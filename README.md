# Docrab
An EDMS (Electronic document managment system) written in rust. I've split the project into a frontend and backend. The backend is written in rust by using Rocket and Diesel. The frontend is written with Sveltekit.
The backend consists of an api an a worker. The idea is, that you send requests to the api, which delegates the work to the workers through RabbitMQ. When a job is complete, the workers then notify the frontend also through RabbitMQ to make it responsive.
Thats the theory for now.

## Goals
The goal of this project is to make a robust and error free API that manages and extracts content from documents and can be run "headless" with or without an frontent interface like a browser.

## My reasons for creating this project
This is the first time I'm seriously making an open source project. I'm learning how exactly git works and I'm learning rust too. 

Also I've used Mayan EDMS, Paperless and Teedy which are all great managment systems, but for my own use, I wanted to integrate documents into an internall application and I found their APIs a bit lacking. I've also had problems with extracting content from documents as I think they're all forked from the same project that I have forgot the name of.

## Contributing
If you'd like, you're welcome to contribute to this project. I'd be very happy if anybody offers help! I'm still planning the project out and currently I don't have clear goals in sight, but if you have any ideas or issues feel free to open up discussions or issues! 
