# SolarHeat

SolarHeat is a web interface designed to control your heat pump based on the angle of the sun.

## How does it work

You host a website on your Raspberry Pi (RPI) to control settings and states of the app.
When the heat pump should run according to your settings a port on the RPI is set to high.
All you have to do is run this Project and connect your RPI in a way to control the heat pump.

## How to use

To run just the control website:
> :warning: You may need root privileges to run the docker container on port 80. If that is not an option replace -p 80:3000 with -p 3000:3000

`docker run -dit --name solarheat -p 8080:8080 -p 80:3000 ghcr.io/felixrieg/solarheat:latest`

To control the RPI GIO pins run:
`curl -o- https://raw.githubusercontent.com/felixrieg/SolarHeat/main/GPIO.sh | bash`

I myself have put something like this in my .bashrc on my RPI:

``` bash
docker container prune --force
sudo docker run -dit --name solarheat -p 8080:8080 -p 80:3000 ghcr.io/felixrieg/solarheat:latest
curl -o- https://raw.githubusercontent.com/felixrieg/SolarHeat/main/GPIO.sh | bash
```

Enjoy the convenience and energy savings that SolarHeat brings to your home heating system. Harness the power of the sun to efficiently control your heat pump and reduce your carbon footprint.
