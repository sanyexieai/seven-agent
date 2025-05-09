import { useState } from 'react';
import ReactMarkdown from 'react-markdown';
import './Chat.css';

export default function Chat() {
  const [messages, setMessages] = useState<{role: 'user'|'assistant', content: string}[]>([]);
  const [input, setInput] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!input.trim()) return;
    setMessages([...messages, {role: 'user', content: input}]);
    setInput('');
    setTimeout(() => {
      setMessages(msgs => [...msgs, {role: 'assistant', content: '**加粗** _斜体_  \n`代码`  \n- 列表1\n- 列表2\n[百度](https://baidu.com)'}]);
    }, 800);
  };

  return (
    <div>
      {/* 顶部导航栏 */}
      <div className="chat-header">
        <div className="chat-header-title">
          <span style={{fontSize: 20}}>☰</span>
          ChatGPT
          <span style={{fontSize: 12, color: '#888'}}>▼</span>
        </div>
        <button className="chat-header-btn">共享</button>
      </div>

      {/* 聊天内容区 */}
      <div className="chat-main">
        {messages.map((msg, i) => (
          <div className={`chat-message ${msg.role}`} key={i}>
            <div className="chat-bubble">
              {msg.role === 'assistant' ? (
                <ReactMarkdown>{msg.content}</ReactMarkdown>
              ) : (
                msg.content
              )}
            </div>
          </div>
        ))}
      </div>

      {/* 底部输入区 */}
      <div className="chat-footer">
        <div className="chat-footer-inner">
          <form onSubmit={handleSubmit}>
            <div className="chat-input-box">
              <input
                className="chat-input"
                value={input}
                onChange={e => setInput(e.target.value)}
                placeholder="询问任何问题"
              />
              <button className="chat-send-btn" type="submit" disabled={!input.trim()}>
                <span style={{fontSize: 18, transform: 'rotate(-160deg)'}}>➤</span>
              </button>
            </div>
            <div className="chat-footer-bar">
              <div>
                <button type="button" className="chat-header-btn" style={{fontSize: 12, padding: '2px 10px'}}>推理</button>
                <button type="button" className="chat-header-btn" style={{fontSize: 12, padding: '2px 10px', marginLeft: 4}}>深度研究</button>
                <button type="button" className="chat-header-btn" style={{fontSize: 12, padding: '2px 10px', marginLeft: 4}}>…</button>
              </div>
              <span className="tips">ChatGPT 也可能会出错。请核查重要信息。</span>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
