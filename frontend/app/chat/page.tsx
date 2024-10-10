"use client";

import { useState, useEffect, useRef } from "react";

// Sample contacts
const contacts = [
  { id: 1, name: "Maria Pepita", lastMessage: "Good morning everybody :)", time: "13:30" },
  { id: 2, name: "Don Pepo", lastMessage: "Good morning everybody :)", time: "13:30" },
  { id: 3, name: "Lalala", lastMessage: "Good morning everybody :)", time: "Yesterday" },
];

export default function ChatPage() {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [messages, setMessages] = useState<string[]>([]);
  const [input, setInput] = useState("");
  const [name, setName] = useState("");
  const [chatRooms, setChatRooms] = useState<string[]>([]);
  const [activeChat, setActiveChat] = useState<any | null>(null); // Active chat window
  const messageEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const socket = new WebSocket("ws://127.0.0.1:8080/api/ws");

    socket.addEventListener("open", () => {
      console.log("Connected to WebSocket");
      setWs(socket);
    });

    socket.addEventListener("message", (event) => {
      const data = event.data as string;
      setMessages((prevMessages) => [...prevMessages, data]);

      if (data.startsWith("Rooms:")) {
        const rooms = data.replace("Rooms:", "").trim().split(",");
        setChatRooms(rooms);
      }
    });

    return () => {
      socket.close();
    };
  }, []);

  useEffect(() => {
    messageEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const handleSendMessage = () => {
    if (input && ws && activeChat) {
      ws.send(input);
      setInput("");
    }
  };

  const handleSetName = () => {
    if (ws) {
      const command = `/name ${name}`;
      ws.send(command);
      setMessages((prevMessages) => [...prevMessages, `You: ${command}`]);
    }
  };

  const openChat = (contact: any) => {
    setActiveChat(contact);
  };

  return (
    <div className="flex h-screen">
      {/* Sidebar - Contact List */}
      <div className="w-1/4 bg-gray-800 p-4">
        <h2 className="text-xl font-bold text-white mb-4">Messages</h2>
        <input
          type="text"
          placeholder="Search messages..."
          className="w-full p-2 bg-gray-700 border border-gray-600 rounded mb-4 text-white"
        />
        <ul>
          {contacts.map((contact) => (
            <li
              key={contact.id}
              className="p-2 hover:bg-gray-700 cursor-pointer rounded"
              onClick={() => openChat(contact)}
            >
              <div className="flex items-center">
                <div className="bg-gray-600 rounded-full h-8 w-8 mr-3"></div>
                <div className="flex-1">
                  <p className="text-white">{contact.name}</p>
                  <p className="text-gray-400 text-sm">{contact.lastMessage}</p>
                </div>
                <span className="text-gray-400 text-sm">{contact.time}</span>
              </div>
            </li>
          ))}
        </ul>
      </div>

      {/* Chat Window (Pop-up Style) */}
      <div className="flex-1 bg-gray-900 p-6">
        {activeChat ? (
          <div className="bg-gray-800 p-4 rounded-lg shadow-lg w-full h-full relative">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-xl text-white">{activeChat.name}</h3>
              <button
                className="bg-red-600 text-white px-3 py-1 rounded"
                onClick={() => setActiveChat(null)}
              >
                Close
              </button>
            </div>

            {/* Messages */}
            <div className="h-64 overflow-y-scroll bg-gray-700 p-4 rounded">
              {messages.map((msg, index) => (
                <div key={index} className="mb-2 text-white">
                  {msg}
                </div>
              ))}
              <div ref={messageEndRef} />
            </div>

            {/* Input Box */}
            <div className="absolute bottom-0 left-0 w-full p-4 bg-gray-700 flex items-center">
              <input
                type="text"
                className="flex-1 p-2 bg-gray-600 border border-gray-500 rounded text-white"
                placeholder="Type a message..."
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyPress={(e) => e.key === "Enter" && handleSendMessage()}
              />
              <button
                className="ml-2 bg-blue-600 text-white px-4 py-2 rounded"
                onClick={handleSendMessage}
              >
                Send
              </button>
            </div>
          </div>
        ) : (
          <div className="flex items-center justify-center h-full text-white">
            <p>Select a contact to start chatting!</p>
          </div>
        )}
      </div>
    </div>
  );
}
