# Axum fileshare

## Для запуска Постгреса и С3 хранилища 

Скопировать .env.override в .env

```bash
docker compose -f .\deploy\docker-compose.yml up
```


## Для запуска сервиса 
```bash
cargo run --release 
```
Сервис открыт на :3000 порту

## Postman

Коллекции Postman V2.1 в папке ./postman 
