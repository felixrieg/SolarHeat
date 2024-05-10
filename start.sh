#!/bin/bash
echo "Starting server and client..."
./start_server.sh &

./client.sh &

while :; 
do 
    response=$(curl -s "http://localhost:8080/status")
    status=$(echo $response | jq -r '.status')
    pin=$(echo $response | jq -r '.pin')
    # echo "Status: $status, Pin: $pin"

    if command -v raspi-gpio &> /dev/null
    then
        echo $(raspi-gpio get $pin)
        echo $(raspi-gpio set $pin op)
        if $status == "true"
        then
            echo "Pin high"
            echo $(raspi-gpio set $pin dh)
        else
            echo "Pin low"
            echo $(raspi-gpio set $pin dl)
        fi
    fi

    sleep 2; 
done
