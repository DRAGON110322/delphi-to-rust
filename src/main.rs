mod serv_messages;

slint::slint! {
    import { Button, VerticalBox, HorizontalBox, TabWidget, StandardTableView, LineEdit, CheckBox, ListView, GroupBox } from "std-widgets.slint";

    export component MainWindow inherits Window {
        title: "Монитор сервера (47073)";
        
        preferred-width: 850px;  
        preferred-height: 600px;
        background: #F0F0F0; 
        
        default-font-size: 12px; 

        in-out property <string> active_dialog: "none";

        in-out property <[[StandardListViewItem]]> projects_data: [
            [ { text: "AppComb.mpr" }, { text: "Идет выполнение" } ],
            [ { text: "ATV.mpr" }, { text: "Идет выполнение" } ],
            [ { text: "Alkali.mpr" }, { text: "Идет выполнение" } ]
        ];

        in-out property <[[StandardListViewItem]]> clients_data: [
            [ { text: "user" }, { text: "Lenovo" }, { text: "21.06.2026 11:56:46" }, { text: "DESKTOP-I0KL42L" }, { text: "127.0.0.1" } ]
        ];

        // Главный контейнер
        VerticalLayout {
            padding: 0px;
            spacing: 0px;

            VerticalLayout {
                padding: 8px;
                spacing: 8px;

                // Верхняя часть (проекты)
                TabWidget {
                    height: 40%;
                    Tab {
                        title: "Проекты";
                        StandardTableView {
                            width: 100%; height: 100%;
                            columns: [
                                { title: "Имя проекта", width: 250px },
                                { title: "Состояние" } 
                            ];
                            rows: root.projects_data;
                        }
                    }
                    Tab { title: "База каналов"; Rectangle {} }
                }

                // Нижняя часть (клиенты)
                TabWidget {
                    Tab {
                        title: "Рабочие станции";
                        StandardTableView {
                            width: 100%; height: 100%;
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
                    Tab { title: "Карманные компьютеры"; Rectangle {} }
                    Tab { title: "Сервисные программы"; Rectangle {} }
                    Tab { title: "Сервера"; Rectangle {} }
                }

                // Серая линия-разделитель
                Rectangle { height: 2px; background: #A0A0A0; }

                // Панель кнопок
                HorizontalLayout {
                    alignment: start;
                    spacing: 5px;
                    Button { text: "Настройки сервера"; width: 140px; height: 24px; clicked => { root.active_dialog = "settings"; } }
                    Button { text: "Список серверов"; width: 120px; height: 24px; clicked => { root.active_dialog = "servers"; } }
                    Button { text: "Статистика"; width: 100px; height: 24px; clicked => { root.active_dialog = "stats"; } }
                    Button { text: "Настройка сервисов"; width: 140px; height: 24px; clicked => { root.active_dialog = "services"; } }
                    Button { text: "Протоколирование"; width: 140px; height: 24px; clicked => { root.active_dialog = "logs"; } }
                }
            }

            // Статус-машины
            Rectangle {
                height: 22px;
                background: #F0F0F0;
                border-color: #A0A0A0;
                border-width: 1px;
                Text {
                    x: 5px;
                    height: 100%;
                    vertical-alignment: center;
                    text: "Имя машины в сети: DESKTOP-I0KL42L";
                    color: #000;
                }
            }
        }

        // Всплывающие окна
        if root.active_dialog != "none" : Rectangle {
            background: rgba(0, 0, 0, 0.4);
            TouchArea {} 

            // Настройки сервера
            if root.active_dialog == "settings" : Rectangle {
                width: min(700px, root.width - 40px);
                height: min(520px, root.height - 40px); 
                background: #F0F0F0; border-color: gray; border-width: 1px;
                
                VerticalLayout {
                    padding: 8px; spacing: 5px;
                    Text { text: "Настройки сервера"; font-weight: 700; height: 20px; }
                    
                    TabWidget {
                        Tab {
                            title: "Общие настройки сервера";
                            VerticalLayout {
                                padding: 10px; alignment: start; spacing: 10px;
                                CheckBox { text: "Автозагрузка с Windows"; }
                                Text { text: "Путь к базе каналов:"; }
                                HorizontalLayout {
                                    spacing: 5px;
                                    LineEdit { text: "Z:\\Monitor\\chbase"; height: 24px; }
                                    Button { text: "..."; width: 30px; height: 24px; }
                                }
                            }
                        }
                        Tab {
                            title: "Настройки проектов";
                            HorizontalLayout {
                                padding: 10px; spacing: 15px;
                                VerticalLayout {
                                    Text { text: "Проекты:"; height: 20px; }
                                    Rectangle {
                                        background: white; border-color: gray; border-width: 1px;
                                        ListView {
                                            for item in ["AppComb.mpr", "ATV.mpr", "Alkali.mpr"]:
                                                Rectangle { height: 20px; Text { text: item; x: 5px; vertical-alignment: center; } }
                                        }
                                    }
                                }
                                VerticalLayout {
                                    alignment: start; spacing: 8px; width: 220px;
                                    CheckBox { text: "Автозагрузка проекта"; checked: true; }
                                    CheckBox { text: "Разрешение загрузки проекта"; checked: true; }
                                    Rectangle { height: 10px; } 
                                    Button { text: "Добавить..."; height: 24px; }
                                    Button { text: "Удалить"; height: 24px; }
                                    Button { text: "Удалить всё"; height: 24px; }
                                }
                            }
                        }
                        
                        Tab { 
                            title: "Права доступа"; 
                            VerticalLayout {
                                padding: 10px; spacing: 15px;
                                HorizontalLayout {
                                    height: 40%; spacing: 15px;
                                    VerticalLayout {
                                        Text { text: "Имена пользователей:"; height: 20px; }
                                        Rectangle {
                                            background: white; border-color: gray; border-width: 1px;
                                            ListView {
                                                for user in ["SUPERVISOR", "USER"]:
                                                    Rectangle { height: 20px; Text { text: user; x: 5px; vertical-alignment: center; } }
                                            }
                                        }
                                    }
                                    VerticalLayout {
                                        spacing: 10px; alignment: start;
                                        HorizontalLayout { spacing: 10px; Text { text: "Пароль:"; vertical-alignment: center; width: 120px; } LineEdit { height: 24px; } }
                                        HorizontalLayout { spacing: 10px; Text { text: "Уровень доступа:"; vertical-alignment: center; width: 120px; } LineEdit { text: "10"; height: 24px; width: 60px; } }
                                        CheckBox { text: "Ограничение редактирования"; }
                                    }
                                }
                                HorizontalLayout {
                                    height: 50%; spacing: 15px;
                                    VerticalLayout {
                                        width: 30%;
                                        Text { text: "Проекты:"; height: 20px; }
                                        Rectangle { background: white; border-color: gray; border-width: 1px; }
                                    }
                                    StandardTableView {
                                        width: 70%;
                                        columns: [
                                            { title: "Имя пользователя", width: 150px },
                                            { title: "Ур. доступа", width: 90px },
                                            { title: "Машины" }
                                        ];
                                        rows: [];
                                    }
                                }
                            }
                        }
                        
                        Tab { 
                            title: "Настройка базы каналов"; 
                            HorizontalLayout {
                                padding: 10px; spacing: 15px;
                                VerticalLayout {
                                    spacing: 8px; width: 50%;
                                    Text { text: "Список узлов базы каналов:"; height: 20px; }
                                    Rectangle {
                                        height: 150px; background: white; border-color: gray; border-width: 1px;
                                        ListView { 
                                            for item in ["Аппаратный"]:
                                                Rectangle { height: 20px; Text { text: item; x: 5px; vertical-alignment: center; } } 
                                        }
                                    }
                                    Button { text: "Добавить..."; height: 24px; }
                                    Button { text: "Удалить"; height: 24px; }
                                    Button { text: "Удалить всё"; height: 24px; }
                                }
                                VerticalLayout {
                                    alignment: start; padding-top: 25px;
                                    CheckBox { text: "Разрешение загрузки узла базы каналов"; checked: true; }
                                }
                            }
                        }
                    }
                    HorizontalLayout { 
                        alignment: end; spacing: 10px; height: 35px;
                        Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } 
                        Button { text: "Отмена"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } 
                    }
                }
            }

            // Список серверов
            if root.active_dialog == "servers" : Rectangle {
                width: min(450px, root.width - 20px); height: min(300px, root.height - 20px); 
                background: #F0F0F0; border-color: gray; border-width: 1px;
                VerticalLayout {
                    padding: 10px; spacing: 5px;
                    Text { text: "Список серверов"; font-weight: 700; height: 20px; }
                    HorizontalLayout {
                        spacing: 15px;
                        Rectangle { width: 50%; background: white; border-color: gray; border-width: 1px; }
                        VerticalLayout {
                            alignment: start; spacing: 10px;
                            Text { text: "Имя сервера"; }
                            LineEdit { height: 24px; }
                            Button { text: "Добавить сервер"; height: 24px; }
                            Button { text: "Удалить сервер"; height: 24px; }
                        }
                    }
                    HorizontalLayout { alignment: center; padding-top: 10px; height: 35px; spacing: 15px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Cancel"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            // Статистика
            if root.active_dialog == "stats" : Rectangle {
                width: min(450px, root.width - 20px); height: 130px; background: #F0F0F0; border-color: gray; border-width: 1px;
                VerticalLayout {
                    padding: 15px; alignment: space-between;
                    Text { text: "Information"; font-weight: 700; }
                    HorizontalLayout {
                        spacing: 15px;
                        Text { text: "i"; font-size: 24px; color: blue; width: 20px; }
                        Text { text: "Connections = 0. NumCreateObj = 278, NumDeleteObj = 271, InitChbaseTags = 0"; wrap: word-wrap; }
                    }
                    HorizontalLayout { alignment: center; Button { text: "OK"; clicked => { root.active_dialog = "none"; } width: 80px; height: 24px; } }
                }
            }

            // Настройка сервисов
            if root.active_dialog == "services" : Rectangle {
                width: min(350px, root.width - 20px); height: min(350px, root.height - 20px); 
                background: #F0F0F0; border-color: gray; border-width: 1px;
                VerticalLayout {
                    padding: 10px; spacing: 10px;
                    Text { text: "Настройка списка сервисов"; font-weight: 700; height: 20px; }
                    Button { text: "Перезагрузка Dll сервисов"; height: 24px; }
                    Rectangle {
                        background: white; border-color: gray; border-width: 1px;
                        ListView {
                            for item in ["1 - buglog.dll", "11 - PGUniServ.dll", "12 - MSUniServ.dll", "2 - ConnectionLog.dll", "3 - propservice.dll", "5 - ClientPLog.dll", "6 - ClientTLog.dll"]:
                                Rectangle { height: 20px; Text { text: item; x: 5px; vertical-alignment: center; } }
                        }
                    }
                    Button { text: "Настройка сервиса"; height: 24px; }
                    HorizontalLayout { alignment: end; spacing: 10px; height: 35px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Cancel"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }

            // Протоколирование
            if root.active_dialog == "logs" : Rectangle {
                width: min(900px, root.width - 20px); height: min(550px, root.height - 20px); 
                background: #F0F0F0; border-color: gray; border-width: 1px;
                VerticalLayout {
                    padding: 10px; spacing: 10px;
                    Text { text: "Настройка протоколирования"; font-weight: 700; height: 20px; }
                    HorizontalLayout {
                        spacing: 15px;
                        StandardTableView {
                            width: 100%; height: 100%;
                            columns: [
                                { title: "№", width: 40px }, { title: "Проект", width: 100px }, { title: "Объект", width: 100px },
                                { title: "Свойство", width: 100px }, { title: "Тэг", width: 80px }, { title: "ID канала", width: 80px },
                                { title: "Дельта времени", width: 120px }, { title: "Дельта", width: 80px }, 
                                { title: "Всегда протоколировать" }
                            ];
                            rows: [];
                        }
                        VerticalLayout {
                            width: 180px; alignment: start; spacing: 5px;
                            Button { text: "Добавить..."; height: 24px; }
                            Button { text: "Удалить"; height: 24px; }
                            Button { text: "Удалить несуществующие"; height: 24px; }
                            Button { text: "Импорт протоколируемых"; height: 24px; }
                            Button { text: "Добавить файлы..."; height: 24px; }
                        }
                    }
                    HorizontalLayout { alignment: end; spacing: 10px; height: 35px; Button { text: "OK"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } Button { text: "Отмена"; width: 80px; height: 24px; clicked => { root.active_dialog = "none"; } } }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = MainWindow::new()?;
    ui.run()
}