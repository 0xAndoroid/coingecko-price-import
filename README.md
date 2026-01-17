# CoinGecko Price Import
This repo contains a simple webserver written in Rust using Rocket to import live prices of cryptocurrencies into Google Sheets  

## Why?
I've tried importing prices into Google sheets directly, but apparently, CoinGecko has placed limitations for
Google's IP addresses because there were a lot of requests (no surprise, google sends all requests from all google sheets from single set of IPs).  
This webserver serves as a middleman between google sheets and coingecko, so that requests to coingecko come from an IP other than Google's.

## How do I run it?
Basically, you should install Rust, clone the repo and use
```shell
cargo build --release
```
You might encounter some errors, and you might need to install some packages, like build-essentials.
Also, if you are compiling on linux, you have to have openssl installed.

## Free endpoint
I am hostig an endpoint on AWS right now (at the time of writing this readme). You can use it, if you want to, but if it  lot of traffic, I'll add auth and limitations.
It's available on
```
http://coingecko.tretyakov.xyz:1234/price
```
You also have to pass two parameters:
- ids_=bitcoin,ethereum (basically, comma separated list of crypto from coingecko names)
- vs_=usd (currencies to which fetch prices, also can be comma separated)

So, full link would look like this
```
http://coingecko.tretyakov.xyz:1234/price?ids_=bitcoin,ethereum&vs_=usd,aud
```


