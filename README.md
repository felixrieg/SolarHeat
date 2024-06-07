# SolarHeat

SolarHeat is a web interface designed to control your heat pump based on the angle of the sun.

## How does it work

You host a website on your Raspberry Pi (RPI) to control settings and states of the app.
When the heat pump should run according to your settings a port on the RPI is set to high.
All you have to do is run this Project and connect your RPI in a way to control the heat pump.

## How to use

To run just the control website:
> :warning: You may need root privileges to run the docker container on port 80. If that is not an option replace -p 80:3000 with -p 3000:3000

``` bash
docker run -dit --pull=always --rm -p 8080:8080 -p 80:3000 ghcr.io/felixrieg/solarheat:latest
```

If you want to keep your settings when restarting your RPI, just add `-v /any/folder/on/your/computer:/storage`, so for me it looks something like this:

``` bash
docker run -dit --pull=always --rm -p 8080:8080 -p 80:3000 -v ~/Git/SolarHeat/server/storage:/storage ghcr.io/felixrieg/solarheat:latest
```

To control the RPI GIO look at my script:
`curl -o- https://raw.githubusercontent.com/felixrieg/SolarHeat/main/run.sh | bash`
This script starts the website and controls the GPIO pins.

I myself have put this in /etc/rc.local on my RPI:

``` bash
curl -o- https://raw.githubusercontent.com/felixrieg/SolarHeat/main/run.sh | bash &
```

Enjoy the convenience and energy savings that SolarHeat brings to your home heating system. Harness the power of the sun to efficiently control your heat pump and reduce your carbon footprint.
