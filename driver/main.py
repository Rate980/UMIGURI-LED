import websocket

ws = websocket.WebSocket()
ws.connect("ws://127.0.0.1:50000")
# ws.send(b"\x01\x12\x23\x28\x00\x12")
ws.send(b'\x01\x11')
for i in ws.recv():
    print(f'{i:X}', end=', ')
print()
ws.close()
