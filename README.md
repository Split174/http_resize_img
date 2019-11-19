Требования:
- Возможность загружать изображения по заданному URL (изображение
размещено где-то в интернете).
- Возможность загружать несколько изображений в одном запросе.
- Создание квадратного превью изображения размером 100px на 100px.


Установка и запуск:

rustup default nightly <br>
git clone https://github.com/Split174/http_resize_img ~/testPopov <br>
cd testPopov <br>
cargo run <br>

Проверка работы: <br>
curl -s -X POST http://localhost:8000/ResizeImage -d \ <br>
"https://pbs.twimg.com/profile_images/832551693/teewu6.png \ <br>
https://media.forgecdn.net/attachments/71/381/reactor.png \ <br>
https://elvortex.com/wp-content/uploads/2013/12/dethklok20logo.jpg" <br>

Сервер вернёт Json с ссылками на превью изображения
