require 'socket'

def handle_client(client)
  puts "New client connected: #{client.peeraddr}"

  loop do
    data = client.recv(1024)
    if data.empty?
      puts "Client disconnected: #{client.peeraddr}"
      return
    end

    puts "Received message: #{data}"
    client.write(data)
  end
end

server = TCPServer.new('127.0.0.1', 7878)
puts "Server listening on port 7878"

loop do
  client = server.accept
  Thread.new { handle_client(client) }
end
