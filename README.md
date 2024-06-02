# Hikari-Bot
Телеграм бот для игры в [Ширитори](https://en.wikipedia.org/wiki/Shiritori)\
Сделан с любовью для клуба Hikari

## Использование
Предназначен для группового чата (лучше выделить в отдельную тему). 

## Используемые словари
+ [Jisho](https://jisho.org/)
+ [JMDict-Yomitan Russian](https://github.com/themoeway/jmdict-yomitan) (в планах)

## Как запускать
Получите токен для Api Telegram. https://t.me/botfather
```sh
git clone https://github.com/bakalover/hikari-bot
docker compose up -d db
```
Для запуска приложения необходимо определить следующие переменные окружения
```toml
PG_HOST=localhost
PG_LOGIN=postgres
PG_PASS=123123
PG_DB=postgres
HIKARI_BOT_TOKEN=<tg token>
```
Когда все переменные определены, можно выполнить
```sh
go run bot.go
```
При запуске приложения в Docker достаточно записать `HIKARI_BOT_TOKEN` в файл `.env` и выполнить
```sh
docker compose up -d --build
```