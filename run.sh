#!/bin/bash

# Start the solarheat container
sudo docker run -dit --pull=always --rm -p 8080:8080 -p 80:3000 -v .:/storage ghcr.io/felixrieg/solarheat:latest &

lastOutput=""
oldPin="25"
oldStatus="false"

CLIENTOUT="/tmp/solarheatclient.out"
echo $CLIENTOUT
echo -e "$(date '+%Y-%m-%d %H:%M:%S') -  Starting client"  >> $CLIENTOUT


# Check if raspi-gpio is installed
if ! command -v raspi-gpio &> /dev/null
then
    echo "raspi-gpio not found, please install it" >> $CLIENTOUT
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null
then
    echo "jq not found, please install it" >> $CLIENTOUT
    exit 1
fi

while :; 
do 
    response=$(curl -s "http://localhost:8080/status")
    status=$(echo $response | jq -r '.status')
    pin=$(echo $response | jq -r '.pin')

    # Check if we got a response
    if [ -z "$response" ]
    then
        newOutput="No response"
    else
        newOutput="Received response: $response"
        if [ "$oldPin" != "$pin" ] || [ "$oldStatus" != "$status" ]
        then
            # Something changed
            if [ "$oldPin" != "$pin" ]
            then
                newOutput="$newOutput\n\tset old pin($oldPin) low"
                raspi-gpio set $pin op d
            fi

            if [ "$status" == "true" ]
            then
                newOutput="$newOutput\n\tset pin($pin) high"
                raspi-gpio set $pin op dh
            else
                newOutput="$newOutput\n\tset pin($pin) low"
                raspi-gpio set $pin op dl
            fi

            oldPin=$pin
            oldStatus=$status
        fi
    fi

    if [ "$lastOutput" != "$newOutput" ] && [ "$newOutput" != "" ]
    then
        echo -e "$(date '+%Y-%m-%d %H:%M:%S') -  $newOutput" >> $CLIENTOUT
        lastOutput=$newOutput
    fi
    sleep 3; 
done
