# что такое веб сервер
Хранить интернет постранично (html) тяжело, поэтому мы собираем странички  по запросу.

![Alt text](image.png)

## Типичный сценарий веб-приложения 
![Alt text](image-1.png)

* сервер - железка с gateway (вебсервером)
* вебсервер(gateway) - приложение, которое умеет слушать какой-то порт и принимать запросы. Передает запрос в приложение (server app)
* server app

Раньше приложение поднималось по запросу и ложилось после ответа. С питоном это не работает, т.к. долго

## Проблемы 
![Alt text](image-2.png)

## План лекций
![Alt text](image-3.png)

WSGI(визги) -- web server gateway interface

![Alt text](image-4.png)

Тут 2 сервера: 
* первый на с++/java (`web server`) может стучаться в python, может сам какую-то статику отдавать. Принимает множество запросов и адресует их куда-то.
* второй -- python с wsgi(например, `gunicorn`, `waitress`), который может дергать питоновские функции. поднят всегда и просто дергает функции по запросу.
* приоложение с `WSGI` дает удобный интерфейс для взаимодейтсивя (н-р, `Django`)

![Alt text](image-5.png)

## Application
![Alt text](image-6.png)

## Example
![Alt text](image-7.png)

1. Возвращать нужно itrable объект (чтобы можно было большие объекты загружать по кусочкам)

2. env хранит
* строку запроса   для ../doc?id=145 будет id=145
* информацию о приложении
* legacy cgi (common gateway interface)
![Alt text](image-8.png)
Django парсит эти данные автоматически и дает нам интерфейс. 

## Server/Gateway
![Alt text](image-9.png)

## Middleware
работает на одной железке с приложением

![Alt text](image-10.png)

Декоратор над приложением 

![Alt text](image-11.png)

Например, для подсчета количества подключений 

![Alt text](image-12.png)

Или для проверки пути

![Alt text](image-13.png)

При вызове нужно явно указывать приложение вызываемо 

![Alt text](image-14.png)

WSGI-сервера

![Alt text](image-15.png)

WSGI App frameworks

![Alt text](image-16.png)

Аналоги для других языков

![Alt text](image-17.png)

# ASGI -- asynchronous server gateway interface

У питона есть GIL (Global Interpreter Lock - глобальная блокировка интерпретатора), который запрещает двум потокам одновременно работать.

(слева gil, справа async io)

![Alt text](image-18.png)

Во время io нагрузки Async передает управление программе, которая может что-то посчитать. 

Отличия от WSGI

![Alt text](image-19.png)

Event loop
у нас есть список задач cpu или IO. Event loop распределяет задачи, перебрасывая их, пока идет ожидание чего-то там.

![Alt text](image-20.png)

## Схема работы 
![Alt text](image-21.png)

HTTP - один раз отправили информацию и получили ответ

Socket -- непрерывный процесс обменна информации, т.е. устанавливается подключение

## Application
![Alt text](image-22.png)

пример 

в scope тело не передается, т.к. оно мб бесконечным (при сокете. например). вместо этого устанавливается флаг more_body в receive. Если мы будем ждать чего-то о receive, то переключаем процесс

информация отправляется дважды: сперва заголовок и всякое такое (потому что оно маленькое) а затем body

![Alt text](image-24.png)

## scope = env

![Alt text](image-23.png)

Каждое сообщение отправляет и принимает тип ивента

![Alt text](image-25.png)

Frameworks

![Alt text](image-26.png)

# Frameworks
![Alt text](image-27.png)

![Alt text](image-28.png)

![Alt text](image-29.png)