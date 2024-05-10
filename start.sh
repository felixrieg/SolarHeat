
echo "Starting server and client..."
./start_server.sh &

./client.sh &

while :; 
do 
    response=$(curl -s "http://localhost:8080/status")
    status=$(echo $response | jq -r '.status')
    pin=$(echo $response | jq -r '.pin')
    echo "Status: $status, Pin: $pin"
    sleep 2; 
done
