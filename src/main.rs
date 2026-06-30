mod serv_messages;
mod config;

use std::cell::RefCell;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use config::{ProjectSettings, load_config, save_config};
use slint::Model;

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, TabWidget, StandardTableView, LineEdit, CheckBox, ListView, GroupBox } from "std-widgets.slint";

    component ContextMenuItem {
        in property <string> text;
        callback clicked;

        min-height: 22px;

        states [
            hovered when touch-area.has-hover : {
                bg.background: #0078d7;
                txt.color: white;
            }
        ]

        bg := Rectangle {
            background: transparent;
            txt := Text {
                text: root.text;
                x: 15px;
                vertical-alignment: center;
                color: black;
                font-size: 11px;
            }
        }

        touch-area := TouchArea {
            clicked => { root.clicked(); }
        }
    }

    export component MainWindow inherits Window {
        callback choose_projects_source();
        callback choose_project_file();
        callback choose_chbase_source();
        callback save_settings();
        callback open_project();
        callback close_project();
        callback restart_project();
        callback delete_selected_project();
        callback set_autoload_for_selected(bool);
        callback set_allow_load_for_selected(bool);
        callback prepare_settings_dialog();
        callback minimize_to_tray();
        callback allow_clients();
        callback disable_clients();
        callback reconnect_clients();
        callback restart_clients();
        callback close_clients();
        callback show_menu();

        title: "Монитор сервера (47073)";
        preferred-width: 800px;
        preferred-height: 550px;
        background: #e9edf3;
        default-font-size: 12px;

        in-out property <string> active_dialog: "none";
        in-out property <string> machine_name: "";
        in-out property <string> chbase_path: "";
        in-out property <bool> autoload_windows: false;
        in-out property <int> selected_project_index: -1;
        in-out property <string> status_text: "";
        in-out property <bool> show_context_menu: false;
        in-out property <length> context_menu_x: 0px;
        in-out property <length> context_menu_y: 0px;
        in-out property <int> context_menu_project_index: -1;
        in-out property <bool> autoload_project_setting: false;
        in-out property <bool> allow_load_project_setting: false;
        in-out property <bool> is_restoring: false;

        in-out property <[[StandardListViewItem]]> projects_data: [];
        in-out property <[[StandardListViewItem]]> chbase_data: [];
        in-out property <[[StandardListViewItem]]> clients_data: [];
        in-out property <[[StandardListViewItem]]> ppc_data: [];
        in-out property <[[StandardListViewItem]]> services_programs_data: [];
        in-out property <[[StandardListViewItem]]> servers_data: [];
        in-out property <[[StandardListViewItem]]> settings_projects_data: [];
        in-out property <[[StandardListViewItem]]> settings_users_data: [];
        in-out property <[[StandardListViewItem]]> settings_access_data: [];
        in-out property <[[StandardListViewItem]]> settings_chbase_data: [];
        in-out property <[[StandardListViewItem]]> services_data: [];
        in-out property <[[StandardListViewItem]]> logs_data: [];

        changed selected_project_index => {
            root.prepare_settings_dialog();
        }

        TouchArea {
            clicked => {
                root.selected-project-index = -1;
                root.show-context-menu = false;
            }
        }

        VerticalLayout {
            padding: 6px;
            spacing: 6px;

            Rectangle {
                background: white;
                border-color: #c7d0da;
                border-width: 1px;
                border-radius: 8px;
                vertical-stretch: 1;

                VerticalLayout {
                    padding: 8px;
                    spacing: 8px;

                    TabWidget {
                        vertical-stretch: 1;

                        Tab {
                            title: "Проекты";
                            Rectangle {
                                background: #f8fafc;
                                border-color: #d9e0ea;
                                border-width: 1px;
                                border-radius: 6px;

                                touch_bg := TouchArea {
                                    pointer-event(event) => {
                                        if (event.button == PointerEventButton.right && event.kind == PointerEventKind.down) {
                                            root.selected-project-index = -1;
                                            root.context-menu-project-index = -1;
                                            root.context-menu-x = touch_bg.mouse-x;
                                            root.context-menu-y = touch_bg.mouse-y;
                                            root.show-context-menu = true;
                                        } else if (event.button == PointerEventButton.left && event.kind == PointerEventKind.down) {
                                            root.selected-project-index = -1;
                                            root.show-context-menu = false;
                                        }
                                    }

                                    VerticalLayout {
                                        spacing: 6px;
                                        padding: 6px;

                                        StandardTableView {
                                            width: 100%;
                                            height: 100%;
                                            columns: [
                                                { title: "Имя проекта", width: 250px },
                                                { title: "Состояние", width: 120px }
                                            ];
                                            rows: root.projects_data;
                                            current-row <=> root.selected_project_index;
                                        }
                                    }
                                }
                            }
                        }

                        Tab {
                            title: "База каналов";
                            Rectangle {
                                background: #f8fafc;
                                border-color: #d9e0ea;
                                border-width: 1px;
                                border-radius: 6px;

                                HorizontalLayout {
                                    padding: 6px;
                                    spacing: 10px;

                                    VerticalLayout {
                                        spacing: 6px;
                                        horizontal-stretch: 1;
                                        Rectangle {
                                            background: white;
                                            border-color: gray;
                                            border-width: 1px;
                                            vertical-stretch: 1;
                                            ListView {
                                                for item in ["1 - База каналов 1", "2 - База каналов 2"]:
                                                    Rectangle { height: 20px; Text { text: item; x: 5px; vertical-alignment: center; } }
                                            }
                                        }
                                        HorizontalLayout {
                                            spacing: 6px;
                                            Button { text: "Загрузить из папки..."; height: 24px; clicked => { root.choose_projects_source(); } }
                                            Button { text: "Удалить"; height: 24px; }
                                            Button { text: "Удалить всё"; height: 24px; }
                                        }
                                    }

                                    VerticalLayout {
                                        width: 220px;
                                        spacing: 8px;
                                        CheckBox { text: "Разрешить загрузку узла"; checked: true; }
                                        CheckBox { text: "Автоматически загружать"; }
                                        Text { text: "Путь к базе каналов:"; }
                                        LineEdit { text: root.chbase_path; height: 24px; }
                                        Button { text: "..."; width: 30px; height: 24px; }
                                    }
                                }
                            }
                        }
                    }

                    TabWidget {
                        vertical-stretch: 1;

                        Tab {
                            title: "Рабочие станции";
                            Rectangle {
                                background: #f8fafc;
                                border-color: #d9e0ea;
                                border-width: 1px;
                                border-radius: 6px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя клиента", width: 130px },
                                        { title: "Имя пользователя", width: 160px },
                                        { title: "Время подключения", width: 160px },
                                        { title: "Имя машины", width: 160px },
                                        { title: "IP машины" }
                                    ];
                                    rows: root.clients_data;
                                }
                            }
                        }

                        Tab {
                            title: "Карманные компьютеры";
                            Rectangle {
                                background: #f8fafc;
                                border-color: #d9e0ea;
                                border-width: 1px;
                                border-radius: 6px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя клиента", width: 140px },
                                        { title: "Имя машины", width: 180px },
                                        { title: "Время подключения", width: 160px }
                                    ];
                                    rows: root.ppc_data;
                                }
                            }
                        }

                        Tab {
                            title: "Сервисные программы";
                            Rectangle {
                                background: #f8fafc;
                                border-color: #d9e0ea;
                                border-width: 1px;
                                border-radius: 6px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя клиента", width: 140px },
                                        { title: "Имя машины", width: 180px },
                                        { title: "Время подключения", width: 160px }
                                    ];
                                    rows: root.services_programs_data;
                                }
                            }
                        }

                        Tab {
                            title: "Сервера";
                            Rectangle {
                                background: #f8fafc;
                                border-color: #d9e0ea;
                                border-width: 1px;
                                border-radius: 6px;

                                StandardTableView {
                                    width: 100%;
                                    height: 100%;
                                    columns: [
                                        { title: "Имя сервера", width: 180px },
                                        { title: "Время подключения", width: 180px }
                                    ];
                                    rows: root.servers_data;
                                }
                            }
                        }
                    }

                    Rectangle { height: 1px; background: #c7d0da; }

                    VerticalLayout {
                        spacing: 4px;
                        HorizontalLayout {
                            spacing: 4px;
                            Button { text: "Настройки сервера"; height: 28px; clicked => { root.prepare_settings_dialog(); root.active_dialog = "settings"; } }
                            Button { text: "Список серверов"; height: 28px; clicked => { root.active_dialog = "servers"; } }
                            Button { text: "Статистика"; height: 28px; clicked => { root.active_dialog = "stats"; } }
                            Button { text: "Настройка сервисов"; height: 28px; clicked => { root.active_dialog = "services"; } }
                            Button { text: "Протоколирование"; height: 28px; clicked => { root.active_dialog = "logs"; } }
                        }
                    }
                }
            }

            Rectangle {
                height: 28px;
                background: #f1f5f9;
                border-color: #c7d0da;
                border-width: 1px;
                border-radius: 8px;

                HorizontalLayout {
                    padding-left: 10px;
                    padding-right: 10px;
                    spacing: 20px;
                    Text { text: "Имя машины в сети: " + root.machine_name; vertical-alignment: center; font-weight: 600; }
                }
            }
        }

        if root.active_dialog != "none" : Rectangle {
            background: rgba(0, 0, 0, 0.45);
            TouchArea { clicked => { root.active_dialog = "none"; } }

            if root.active_dialog == "settings" : Rectangle {
                width: min(780px, root.width - 10px);
                height: min(530px, root.height - 10px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: #f8fafc;
                border-color: #9aa6b2;
                border-width: 1px;
                border-radius: 10px;

                VerticalLayout {
                    padding: 12px;
                    spacing: 10px;

                    Text { text: "Настройки сервера"; font-weight: 700; font-size: 14px; height: 22px; }

                    TabWidget {
                        vertical-stretch: 1;
                        Tab {
                            title: "Общие настройки сервера";
                            VerticalLayout {
                                padding: 12px;
                                spacing: 10px;
                                CheckBox { text: "Автозагрузка с Windows"; checked <=> root.autoload_windows; }
                                HorizontalLayout {
                                    spacing: 10px;
                                    Text { text: "Путь к базе каналов:"; vertical-alignment: center; width: 180px; }
                                    LineEdit { text <=> root.chbase_path; height: 24px; }
                                    Button { text: "..."; width: 30px; height: 24px; clicked => { root.choose_chbase_source(); } }
                                }
                            }
                        }

                        Tab {
                            title: "Настройки проектов";
                            HorizontalLayout {
                                padding: 8px;
                                spacing: 8px;
                                vertical-stretch: 1;

                                VerticalLayout {
                                    spacing: 8px;
                                    horizontal-stretch: 1;
                                    vertical-stretch: 1;
                                    Text { text: "Проекты:"; height: 20px; }
                                    Rectangle {
                                        background: white;
                                        border-color: gray;
                                        border-width: 1px;
                                        border-radius: 6px;
                                        vertical-stretch: 1;
                                        StandardTableView {
                                            width: 100%;
                                            height: 100%;
                                            columns: [
                                                { title: "Проект", width: 240px },
                                                { title: "Автозагрузка", width: 120px },
                                                { title: "Разрешение" }
                                            ];
                                            rows: root.settings_projects_data;
                                            current-row <=> root.selected_project_index;
                                        }
                                    }
                                }

                                VerticalLayout {
                                    alignment: start;
                                    spacing: 8px;
                                    width: 220px;
                                    Text {
                                        text: root.selected_project_index >= 0 ? "Параметры проекта:" : "Проект не выбран";
                                        height: 20px;
                                        color: root.selected_project_index >= 0 ? #222 : #999;
                                        font-weight: 600;
                                    }
                                    CheckBox {
                                        text: "Автозагрузка";
                                        enabled: root.selected_project_index >= 0;
                                        checked <=> root.autoload_project_setting;
                                        toggled => {
                                            if (root.selected_project_index >= 0) {
                                                root.set_autoload_for_selected(self.checked);
                                            }
                                        }
                                    }
                                    CheckBox {
                                        text: "Разрешение загрузки";
                                        enabled: root.selected_project_index >= 0 && !root.autoload_project_setting;
                                        checked: root.autoload_project_setting || root.allow_load_project_setting;
                                        toggled => {
                                            if (root.selected_project_index >= 0 && !root.autoload_project_setting) {
                                                root.allow_load_project_setting = self.checked;
                                                root.set_allow_load_for_selected(self.checked);
                                            }
                                        }
                                    }
                                    Rectangle { height: 8px; }
                                    Button {
                                        text: "Добавить...";
                                        height: 24px;
                                        clicked => { root.choose_project_file(); }
                                    }
                                    Button {
                                        text: "Удалить";
                                        height: 24px;
                                        enabled: root.selected_project_index >= 0;
                                        clicked => { root.delete_selected_project(); }
                                    }
                                    Button {
                                        text: "Удалить всё";
                                        height: 24px;
                                        clicked => {
                                            root.settings_projects_data = [];
                                            root.selected_project_index = -1;
                                        }
                                    }
                                }
                            }
                        }

                        Tab {
                            title: "Права доступа";
                            VerticalLayout {
                                padding: 8px;
                                spacing: 8px;
                                vertical-stretch: 1;

                                HorizontalLayout {
                                    spacing: 8px;
                                    vertical-stretch: 1;
                                    VerticalLayout {
                                        spacing: 8px;
                                        width: 280px;
                                        vertical-stretch: 1;
                                        Text { text: "Имена пользователей:"; height: 20px; }
                                        Rectangle {
                                            background: white;
                                            border-color: gray;
                                            border-width: 1px;
                                            border-radius: 6px;
                                            vertical-stretch: 1;
                                            StandardTableView {
                                                width: 100%;
                                                height: 100%;
                                                columns: [
                                                    { title: "Пользователь", width: 140px },
                                                    { title: "Ур. доступа", width: 90px },
                                                    { title: "Описание" }
                                                ];
                                                rows: root.settings_users_data;
                                            }
                                        }
                                    }
                                    VerticalLayout {
                                        spacing: 10px;
                                        alignment: start;
                                        horizontal-stretch: 1;
                                        HorizontalLayout { spacing: 10px; Text { text: "Пароль:"; vertical-alignment: center; width: 120px; } LineEdit { height: 24px; } }
                                        HorizontalLayout { spacing: 10px; Text { text: "Уровень доступа:"; vertical-alignment: center; width: 120px; } LineEdit { text: "10"; height: 24px; width: 60px; } }
                                        CheckBox { text: "Ограничение редактирования"; }
                                    }
                                }

                                HorizontalLayout {
                                    spacing: 8px;
                                    vertical-stretch: 1;
                                    VerticalLayout {
                                        width: 280px;
                                        spacing: 8px;
                                        vertical-stretch: 1;
                                        Text { text: "Проекты:"; height: 20px; }
                                        Rectangle {
                                            background: white;
                                            border-color: gray;
                                            border-width: 1px;
                                            border-radius: 6px;
                                            vertical-stretch: 1;
                                            StandardTableView {
                                                width: 100%;
                                                height: 100%;
                                                columns: [
                                                    { title: "Проект", width: 180px },
                                                    { title: "Доступ" }
                                                ];
                                                rows: root.settings_access_data;
                                            }
                                        }
                                    }
                                    Rectangle {
                                        horizontal-stretch: 1;
                                        background: white;
                                        border-color: gray;
                                        border-width: 1px;
                                        border-radius: 6px;
                                        StandardTableView {
                                            width: 100%;
                                            height: 100%;
                                            columns: [
                                                { title: "Имя пользователя", width: 150px },
                                                { title: "Ур. доступа", width: 90px },
                                                { title: "Машины" }
                                            ];
                                            rows: root.settings_access_data;
                                        }
                                    }
                                }
                            }
                        }

                        Tab {
                            title: "Настройка базы каналов";
                            HorizontalLayout {
                                padding: 12px;
                                spacing: 12px;
                                vertical-stretch: 1;
                                VerticalLayout {
                                    spacing: 8px;
                                    horizontal-stretch: 1;
                                    vertical-stretch: 1;
                                    Text { text: "Список узлов базы каналов:"; height: 20px; }
                                    Rectangle {
                                        background: white;
                                        border-color: gray;
                                        border-width: 1px;
                                        border-radius: 6px;
                                        vertical-stretch: 1;
                                        StandardTableView {
                                            width: 100%;
                                            height: 100%;
                                            columns: [
                                                { title: "Узел", width: 200px },
                                                { title: "Можно загрузить" }
                                            ];
                                            rows: root.settings_chbase_data;
                                        }
                                    }
                                    Button {
                                        text: "Добавить...";
                                        height: 24px;
                                        clicked => {
                                            root.settings_chbase_data = [
                                                [ { text: "Аппаратный" }, { text: "Да" } ],
                                                [ { text: "Резервный" }, { text: "Да" } ],
                                                [ { text: "Тестовый" }, { text: "Нет" } ]
                                            ];
                                        }
                                    }
                                    Button {
                                        text: "Удалить";
                                        height: 24px;
                                        clicked => {
                                            root.settings_chbase_data = [ [ { text: "Аппаратный" }, { text: "Да" } ] ];
                                        }
                                    }
                                    Button {
                                        text: "Удалить всё";
                                        height: 24px;
                                        clicked => {
                                            root.settings_chbase_data = [];
                                        }
                                    }
                                }
                                VerticalLayout {
                                    alignment: start;
                                    padding-top: 24px;
                                    CheckBox { text: "Разрешение загрузки узла базы каналов"; checked: true; }
                                    CheckBox { text: "Автозагрузка узлов"; }
                                }
                            }
                        }
                    }

                    HorizontalLayout {
                        alignment: end;
                        spacing: 10px;
                        height: 35px;
                        Button { text: "OK"; width: 80px; height: 24px; clicked => { root.save_settings(); root.active_dialog = "none"; } }
                        Button { text: "Отмена"; width: 80px; height: 24px; clicked => {
                            root.active_dialog = "none";
                        } }
                    }
                }
            }

            if root.active_dialog == "servers" : Rectangle {
                width: min(500px, root.width - 20px);
                height: min(320px, root.height - 20px);
                    x: (root.width - self.width) / 2;
                    y: (root.height - self.height) / 2;
                background: #f8fafc;
                border-color: #9aa6b2;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    padding: 12px;
                    spacing: 10px;
                    Text { text: "Список серверов"; font-weight: 700; height: 20px; }
                    HorizontalLayout {
                        spacing: 12px;
                        Rectangle {
                            width: 50%;
                            background: white;
                            border-color: gray;
                            border-width: 1px;
                            border-radius: 6px;
                            StandardTableView {
                                width: 100%;
                                height: 100%;
                                columns: [
                                    { title: "Имя сервера", width: 180px },
                                    { title: "Время подключения" }
                                ];
                                rows: root.servers_data;
                            }
                        }
                        VerticalLayout {
                            alignment: start;
                            spacing: 10px;
                            Text { text: "Имя сервера"; }
                            LineEdit { height: 24px; }
                            Button { text: "Добавить сервер"; height: 24px; clicked => { root.servers_data = [ [ { text: "Server-Alpha" }, { text: "21.06.2026 11:56:46" } ], [ { text: "Server-Beta" }, { text: "21.06.2026 11:58:08" } ], [ { text: "Server-Gamma" }, { text: "21.06.2026 12:03:22" } ] ]; } }
                            Button { text: "Удалить сервер"; height: 24px; clicked => { root.servers_data = [ [ { text: "Server-Alpha" }, { text: "21.06.2026 11:56:46" } ] ]; } }
                        }
                    }
                    HorizontalLayout { alignment: center; padding-top: 10px; height: 35px; spacing: 15px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Cancel"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            if root.active_dialog == "stats" : Rectangle {
                width: min(500px, root.width - 20px);
                height: 170px;
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: #f8fafc;
                border-color: #9aa6b2;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    padding: 14px;
                    spacing: 12px;
                    Text { text: "Information"; font-weight: 700; }
                    HorizontalLayout {
                        spacing: 15px;
                        Text { text: "i"; font-size: 24px; color: blue; width: 24px; }
                        Text { text: "Connections = 0. NumCreateObj = 278, NumDeleteObj = 271, InitChbaseTags = 0"; wrap: word-wrap; }
                    }
                    HorizontalLayout { alignment: center; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            if root.active_dialog == "services" : Rectangle {
                width: min(420px, root.width - 20px);
                height: min(380px, root.height - 20px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: #f8fafc;
                border-color: #9aa6b2;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    padding: 12px;
                    spacing: 10px;
                    Text { text: "Настройка списка сервисов"; font-weight: 700; height: 20px; }
                    Button { text: "Перезагрузка Dll сервисов"; height: 24px; clicked => { root.services_data = [ [ { text: "1 - buglog.dll" }, { text: "Перезагружен" } ], [ { text: "11 - PGUniServ.dll" }, { text: "Перезагружен" } ], [ { text: "12 - MSUniServ.dll" }, { text: "Перезагружен" } ], [ { text: "2 - ConnectionLog.dll" }, { text: "Перезагружен" } ], [ { text: "3 - propservice.dll" }, { text: "Перезагружен" } ], [ { text: "5 - ClientPLog.dll" }, { text: "Перезагружен" } ], [ { text: "6 - ClientTLog.dll" }, { text: "Перезагружен" } ] ]; } }
                    Rectangle {
                        background: white;
                        border-color: gray;
                        border-width: 1px;
                        border-radius: 6px;
                        StandardTableView {
                            width: 100%;
                            height: 100%;
                            columns: [
                                { title: "Dll", width: 220px },
                                { title: "Состояние" }
                            ];
                            rows: root.services_data;
                        }
                    }
                    Button { text: "Настройка сервиса"; height: 24px; }
                    HorizontalLayout { alignment: end; spacing: 10px; height: 35px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Cancel"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            if root.active_dialog == "logs" : Rectangle {
                width: min(760px, root.width - 20px);
                height: min(510px, root.height - 20px);
                x: (root.width - self.width) / 2;
                y: (root.height - self.height) / 2;
                background: #f8fafc;
                border-color: #9aa6b2;
                border-width: 1px;
                border-radius: 10px;
                VerticalLayout {
                    padding: 12px;
                    spacing: 10px;
                    Text { text: "Настройка протоколирования"; font-weight: 700; height: 20px; }
                    HorizontalLayout {
                        spacing: 12px;
                        Rectangle {
                            horizontal-stretch: 1;
                            background: white;
                            border-color: gray;
                            border-width: 1px;
                            border-radius: 6px;
                            StandardTableView {
                                width: 100%;
                                height: 100%;
                                columns: [
                                    { title: "№", width: 40px },
                                    { title: "Проект", width: 100px },
                                    { title: "Объект", width: 100px },
                                    { title: "Свойство", width: 100px },
                                    { title: "Тэг", width: 80px },
                                    { title: "ID канала", width: 80px },
                                    { title: "Дельта времени", width: 120px },
                                    { title: "Дельта", width: 80px },
                                    { title: "Всегда протоколировать" }
                                ];
                                rows: root.logs_data;
                            }
                        }
                        VerticalLayout {
                            width: 180px;
                            alignment: start;
                            spacing: 6px;
                            Button { text: "Добавить..."; height: 24px; clicked => { root.logs_data = [ [ { text: "1" }, { text: "AppComb.mpr" }, { text: "Valve" }, { text: "State" }, { text: "Tag001" }, { text: "12" }, { text: "00:00:10" }, { text: "0.1" }, { text: "True" } ], [ { text: "2" }, { text: "ATV.mpr" }, { text: "Pump" }, { text: "Pressure" }, { text: "Tag002" }, { text: "8" }, { text: "00:00:05" }, { text: "1.0" }, { text: "False" } ], [ { text: "3" }, { text: "NewProject.mpr" }, { text: "Tank" }, { text: "Level" }, { text: "Tag003" }, { text: "15" }, { text: "00:00:30" }, { text: "0.5" }, { text: "True" } ] ]; } }
                            Button { text: "Удалить"; height: 24px; clicked => { root.logs_data = [ [ { text: "1" }, { text: "AppComb.mpr" }, { text: "Valve" }, { text: "State" }, { text: "Tag001" }, { text: "12" }, { text: "00:00:10" }, { text: "0.1" }, { text: "True" } ] ]; } }
                            Button { text: "Удалить несуществующие"; height: 24px; }
                            Button { text: "Импорт протоколируемых"; height: 24px; clicked => { root.logs_data = [ [ { text: "1" }, { text: "AppComb.mpr" }, { text: "Valve" }, { text: "State" }, { text: "Tag001" }, { text: "12" }, { text: "00:00:10" }, { text: "0.1" }, { text: "True" } ], [ { text: "2" }, { text: "ATV.mpr" }, { text: "Pump" }, { text: "Pressure" }, { text: "Tag002" }, { text: "8" }, { text: "00:00:05" }, { text: "1.0" }, { text: "False" } ], [ { text: "4" }, { text: "Imported.mpr" }, { text: "Flow" }, { text: "Value" }, { text: "Tag004" }, { text: "18" }, { text: "00:01:00" }, { text: "0.2" }, { text: "True" } ] ]; } }
                            Button { text: "Добавить файлы..."; height: 24px; }
                        }
                    }
                    HorizontalLayout { alignment: end; spacing: 10px; height: 35px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Отмена"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }
        }

        if root.show_context_menu : Rectangle {
            background: rgba(0, 0, 0, 0.02);
            TouchArea { clicked => { root.show_context_menu = false; } }

            Rectangle {
                x: root.context_menu_x;
                y: root.context_menu_y;
                width: 220px;
                background: #f0f0f0;
                border-color: #999;
                border-width: 1px;
                border-radius: 2px;

                VerticalLayout {
                    padding: 2px;
                    spacing: 1px;

                    ContextMenuItem {
                        text: "Открыть проект";
                        clicked => {
                            root.open_project();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Закрыть проект";
                        clicked => {
                            root.close_project();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Перезагрузить проект";
                        clicked => {
                            root.restart_project();
                            root.show_context_menu = false;
                        }
                    }

                    Rectangle {
                        height: 7px;
                        VerticalLayout {
                            padding-top: 3px;
                            padding-bottom: 3px;
                            Rectangle {
                                height: 1px;
                                background: #ccc;
                            }
                        }
                    }

                    ContextMenuItem {
                        text: "Приостановить выполнение";
                        clicked => {
                            root.show_context_menu = false;
                        }
                    }

                    Rectangle {
                        height: 7px;
                        VerticalLayout {
                            padding-top: 3px;
                            padding-bottom: 3px;
                            Rectangle {
                                height: 1px;
                                background: #ccc;
                            }
                        }
                    }

                    ContextMenuItem {
                        text: "Запретить подключение клиентов";
                        clicked => {
                            root.allow_clients();
                            root.show_context_menu = false;
                        }
                    }

                    Rectangle {
                        height: 7px;
                        VerticalLayout {
                            padding-top: 3px;
                            padding-bottom: 3px;
                            Rectangle {
                                height: 1px;
                                background: #ccc;
                            }
                        }
                    }

                    ContextMenuItem {
                        text: "Подключить клиентов";
                        clicked => {
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Отключить клиентов";
                        clicked => {
                            root.disable_clients();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Переподключить клиентов";
                        clicked => {
                            root.reconnect_clients();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Перезагрузить клиентов";
                        clicked => {
                            root.restart_clients();
                            root.show_context_menu = false;
                        }
                    }
                    ContextMenuItem {
                        text: "Закрыть клиентов";
                        clicked => {
                            root.close_clients();
                            root.show_context_menu = false;
                        }
                    }
                }
            }
        }
    }
}

type RowModel = slint::ModelRc<slint::StandardListViewItem>;

fn file_name_text(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .unwrap_or_else(|| path.display().to_string())
}

fn make_row(items: &[&str]) -> RowModel {
    slint::ModelRc::new(slint::VecModel::from(
        items.iter().map(|item| slint::StandardListViewItem::from(*item)).collect::<Vec<_>>()
    ))
}

fn main() -> Result<(), slint::PlatformError> {
    let config = load_config();

    let ui = MainWindow::new()?;
    let weak_ui = ui.as_weak();

    let tray_result = tray_item::TrayItem::new("MonitorServer", "icon");
    let mut tray = match tray_result {
        Ok(t) => Some(t),
        Err(e) => {
            println!("Ошибка создания трея {:?}", e);
            None
        }
    };

    if let Some(ref mut t) = tray {
        let w_open = weak_ui.clone();
        let _ = t.add_menu_item("Open", move || {
            let inner_open = w_open.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(win) = inner_open.upgrade() {
                    win.set_is_restoring(true);
                    let _ = win.show();
                    win.window().set_minimized(false);

                    let win_weak = inner_open.clone();
                    slint::Timer::single_shot(std::time::Duration::from_millis(1000), move || {
                        if let Some(w) = win_weak.upgrade() {
                            w.set_is_restoring(false);
                        }
                    });
                }
            });
        });

        let _ = t.add_menu_item("Exit", move || {
            std::process::exit(0);
        });
    }

    ui.window().on_close_requested(move || {
        std::process::exit(0);
    });

    let ui_handle = weak_ui.clone();
    let timer = slint::Timer::default();
    timer.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(250), move || {
        if let Some(ui) = ui_handle.upgrade() {
            if ui.window().is_minimized() {
                if !ui.get_is_restoring() {
                    let _ = ui.window().hide();
                }
            }
        }
    });

    ui.set_autoload_windows(config.autoload_windows);
    ui.set_machine_name(config.machine_name.clone().into());
    ui.set_chbase_path(config.chbase_path.clone().into());

    let project_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));
    let settings_project_rows: Rc<RefCell<Vec<RowModel>>> = Rc::new(RefCell::new(Vec::new()));

    for project in &config.projects {
        let autoload_str = if project.autoload { "Да" } else { "Нет" };
        let allow_str = if project.allow_load { "Да" } else { "Нет" };

        settings_project_rows.borrow_mut().push(make_row(&[&project.name, autoload_str, allow_str]));

        if project.autoload {
            project_rows.borrow_mut().push(make_row(&[&project.name, "Остановлен"]));
        }
    }

    ui.set_projects_data(slint::ModelRc::new(slint::VecModel::from(project_rows.borrow().clone())));
    ui.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(settings_project_rows.borrow().clone())));

    ui.on_choose_projects_source({
        let weak_ui = weak_ui.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };

            let mut dialog = rfd::FileDialog::new();
            let chbase_path = window.get_chbase_path();
            let chbase_path = Path::new(&chbase_path);
            if chbase_path.exists() {
                dialog = dialog.set_directory(chbase_path);
            }

            let Some(path) = dialog.pick_folder() else {
                return;
            };

            let folder_name = file_name_text(&path);
            let mut entries = Vec::new();
            if let Ok(read_dir) = fs::read_dir(&path) {
                for entry in read_dir.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        entries.push(name.to_string());
                    }
                }
            }

            window.set_chbase_path(path.display().to_string().into());
            let mut rows: Vec<RowModel> = Vec::new();
            if entries.is_empty() {
                rows.push(make_row(&[folder_name.as_str(), "Да"]));
            } else {
                for name in entries {
                    rows.push(make_row(&[name.as_str(), "Да"]));
                }
            }
            window.set_settings_chbase_data(slint::ModelRc::new(slint::VecModel::from(rows)));
        }
    });

    ui.on_choose_project_file({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };

            let mut dialog = rfd::FileDialog::new();
            let chbase_path = window.get_chbase_path();
            let chbase_path = Path::new(&chbase_path);
            if chbase_path.exists() {
                dialog = dialog.set_directory(chbase_path);
            }

            let Some(path) = dialog.pick_file() else {
                return;
            };

            let file_name = file_name_text(&path);

            let (new_projects, new_settings) = {
                let mut p_rows = project_rows.borrow_mut();
                let mut s_rows = settings_project_rows.borrow_mut();
                p_rows.push(make_row(&[file_name.as_str(), "Остановлен"]));
                s_rows.push(make_row(&[file_name.as_str(), "Нет", "Нет"]));
                (p_rows.clone(), s_rows.clone())
            };

            window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_projects)));
            window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
        }
    });

    ui.on_delete_selected_project({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;

                let (new_settings, new_projects) = {
                    let mut s_rows = settings_project_rows.borrow_mut();
                    let mut p_rows = project_rows.borrow_mut();

                    let removed_name_opt = s_rows.get(idx_usize).and_then(|r| r.row_data(0)).map(|it| it.text.to_string());
                    if let Some(name) = removed_name_opt {
                        s_rows.remove(idx_usize);
                        p_rows.retain(|r| {
                            r.row_data(0).map(|it| it.text.to_string() != name).unwrap_or(true)
                        });
                    }
                    (s_rows.clone(), p_rows.clone())
                };

                window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_projects)));
                window.set_selected_project_index(-1);
            }
        }
    });

    ui.on_set_autoload_for_selected({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        let project_rows = project_rows.clone();
        move |flag: bool| {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;

                let (new_settings, new_projects) = {
                    let mut s_rows = settings_project_rows.borrow_mut();
                    let mut p_rows = project_rows.borrow_mut();

                    let (name, allow) = {
                        if let Some(row) = s_rows.get(idx_usize) {
                            let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                            let allow = row.row_data(2).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                            (name, allow)
                        } else {
                            return;
                        }
                    };

                    let autoload_str = if flag { "Да" } else { "Нет" };
                    let allow_str = if allow { "Да" } else { "Нет" };

                    s_rows[idx_usize] = make_row(&[name.as_str(), autoload_str, allow_str]);

                    if flag {
                        let exists = p_rows.iter().any(|r| r.row_data(0).map(|it| it.text.to_string() == name).unwrap_or(false));
                        if !exists {
                            p_rows.push(make_row(&[name.as_str(), "Остановлен"]));
                        }
                    } else {
                        p_rows.retain(|r| r.row_data(0).map(|it| it.text.to_string() != name).unwrap_or(true));
                    }

                    (s_rows.clone(), p_rows.clone())
                };

                window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_projects)));
            }
        }
    });

    ui.on_set_allow_load_for_selected({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        move |flag: bool| {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;

                let new_settings = {
                    let mut s_rows = settings_project_rows.borrow_mut();

                    let (name, autoload) = {
                        if let Some(row) = s_rows.get(idx_usize) {
                            let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                            let autoload = row.row_data(1).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                            (name, autoload)
                        } else {
                            return;
                        }
                    };

                    let autoload_str = if autoload { "Да" } else { "Нет" };
                    let allow_str = if flag { "Да" } else { "Нет" };

                    s_rows[idx_usize] = make_row(&[name.as_str(), autoload_str, allow_str]);
                    s_rows.clone()
                };

                window.set_settings_projects_data(slint::ModelRc::new(slint::VecModel::from(new_settings)));
            }
        }
    });

    ui.on_prepare_settings_dialog({
        let weak_ui = weak_ui.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else { return; };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let s_rows = settings_project_rows.borrow();
                if let Some(row) = s_rows.get(idx_usize) {
                    let autoload = row.row_data(1).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                    let allow = row.row_data(2).map(|it| it.text.to_string() == "Да").unwrap_or(false);
                    window.set_autoload_project_setting(autoload);
                    window.set_allow_load_project_setting(allow);
                } else {
                    window.set_autoload_project_setting(false);
                    window.set_allow_load_project_setting(false);
                }
            } else {
                window.set_autoload_project_setting(false);
                window.set_allow_load_project_setting(false);
            }
        }
    });

    ui.on_choose_chbase_source({
        let weak_ui = weak_ui.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };

            let mut dialog = rfd::FileDialog::new();
            let chbase_path = window.get_chbase_path();
            let chbase_path = Path::new(&chbase_path);
            if chbase_path.exists() {
                dialog = dialog.set_directory(chbase_path);
            }

            let Some(path) = dialog.pick_folder() else {
                return;
            };

            let folder_name = file_name_text(&path);
            let mut entries = Vec::new();
            if let Ok(read_dir) = fs::read_dir(&path) {
                for entry in read_dir.flatten() {
                    if let Some(name) = entry.file_name().to_str() {
                        entries.push(name.to_string());
                    }
                }
            }

            window.set_chbase_path(path.display().to_string().into());
            let mut rows: Vec<RowModel> = Vec::new();
            if entries.is_empty() {
                rows.push(make_row(&[folder_name.as_str(), "Да"]));
            } else {
                for name in entries {
                    rows.push(make_row(&[name.as_str(), "Да"]));
                }
            }
            window.set_settings_chbase_data(slint::ModelRc::new(slint::VecModel::from(rows)));
        }
    });

    ui.on_save_settings({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        let settings_project_rows = settings_project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };

            let mut config = load_config();
            config.autoload_windows = window.get_autoload_windows();
            config.chbase_path = window.get_chbase_path().to_string();

            let mut projects = Vec::new();
            let selected_idx = window.get_selected_project_index();

            {
                let s_rows = settings_project_rows.borrow();
                for (i, row_model) in s_rows.iter().enumerate() {
                    let row = row_model.clone();
                    if let Some(name_item) = row.row_data(0) {
                        let name = name_item.text.to_string();

                        let (autoload, allow_load) = if (i as i32) == selected_idx {
                            (window.get_autoload_project_setting(), window.get_allow_load_project_setting())
                        } else {
                            (row.row_data(1).map(|item| item.text.to_string() == "Да").unwrap_or(false),
                             row.row_data(2).map(|item| item.text.to_string() == "Да").unwrap_or(false))
                        };

                        projects.push(ProjectSettings {
                            name,
                            autoload,
                            allow_load,
                            available_for_clients: false,
                        });
                    }
                }
            }

            config.projects = projects;

            let _ = save_config(&config);

            let new_projects = {
                let mut p_rows = project_rows.borrow_mut();
                p_rows.clear();
                for project in &config.projects {
                    if project.autoload {
                        p_rows.push(make_row(&[&project.name, "Остановлен"]));
                    }
                }
                p_rows.clone()
            };

            window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_projects)));
        }
    });

    ui.on_open_project({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };
            let idx = window.get_context_menu_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_rows = {
                    let mut p_rows = project_rows.borrow_mut();
                    if let Some(row) = p_rows.get(idx_usize) {
                        let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                        p_rows[idx_usize] = make_row(&[name.as_str(), "Работает"]);
                    }
                    p_rows.clone()
                };
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_rows)));
            }
        }
    });

    ui.on_close_project({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };
            let idx = window.get_context_menu_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_rows = {
                    let mut p_rows = project_rows.borrow_mut();
                    if let Some(row) = p_rows.get(idx_usize) {
                        let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                        p_rows[idx_usize] = make_row(&[name.as_str(), "Остановлен"]);
                    }
                    p_rows.clone()
                };
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_rows)));
            }
        }
    });

    ui.on_restart_project({
        let weak_ui = weak_ui.clone();
        let project_rows = project_rows.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };
            let idx = window.get_context_menu_project_index();
            if idx >= 0 {
                let idx_usize = idx as usize;
                let new_rows = {
                    let mut p_rows = project_rows.borrow_mut();
                    if let Some(row) = p_rows.get(idx_usize) {
                        let name = row.row_data(0).map(|it| it.text.to_string()).unwrap_or_default();
                        p_rows[idx_usize] = make_row(&[name.as_str(), "Перезапуск"]);
                    }
                    p_rows.clone()
                };
                window.set_projects_data(slint::ModelRc::new(slint::VecModel::from(new_rows)));
            }
        }
    });

    ui.on_allow_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_disable_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_reconnect_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_restart_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_close_clients({
        let _weak_ui = weak_ui.clone();
        move || {}
    });

    ui.on_show_menu({
        let weak_ui = weak_ui.clone();
        move || {
            let Some(window) = weak_ui.upgrade() else {
                return;
            };
            let idx = window.get_selected_project_index();
            if idx >= 0 {
                window.set_show_context_menu(true);
                window.set_context_menu_x(100.0);
                window.set_context_menu_y(100.0);
                window.set_context_menu_project_index(idx);
            }
        }
    });

    ui.show()?;

    slint::run_event_loop_until_quit()?;
    Ok(())
}