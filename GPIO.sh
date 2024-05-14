#!/bin/bash

while :; 
do 
    response=$(curl -s "http://localhost:8080/status")
    status=$(echo $response | jq -r '.status')
    pin=$(echo $response | jq -r '.pin')

    // Check if we got a response
    if [ -z "$response" ]
    then
        echo "No response"
    else
        echo "Recieved response - Status: $status, Pin: $pin"
        
        // Check if raspi-gpio is installed
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
    fi

    sleep 2; 
done
