import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface BucketItem {
  id: string;
  title: string;
  category: string;
  status: string;
  created_at: string;
  last_touched_at: string;
  future_message: string;
}

function App() {
  const [items, setItems] = useState<BucketItem[]>([]);
  const [title, setTitle] = useState("");
  const [category, setCategory] = useState("旅・冒険");
  const [futureMessage, setFutureMessage] = useState("");
  const [showPopup, setShowPopup] = useState<string | null>(null);

  useEffect(() => {
    loadItems();
  }, []);

  async function loadItems() {
    try {
      const result: BucketItem[] = await invoke("get_items");
      setItems(result);
    } catch (e) {
      console.error(e);
    }
  }

  async function addItem() {
    if (!title) return;
    try {
      await invoke("add_item", { title, category, futureMessage });
      setTitle("");
      setFutureMessage("");
      loadItems();
    } catch (e) {
      console.error(e);
    }
  }

  async function completeItem(item: BucketItem) {
    try {
      await invoke("update_item_status", { id: item.id, status: "completed" });
      setShowPopup(item.future_message);
      loadItems();
    } catch (e) {
      console.error(e);
    }
  }

  async function touchItem(id: string) {
    try {
      await invoke("touch_item", { id });
      loadItems();
    } catch (e) {
      console.error(e);
    }
  }

  const calculateAging = (lastTouched: string) => {
    const last = new Date(lastTouched).getTime();
    const now = new Date().getTime();
    const diffDays = (now - last) / (1000 * 60 * 60 * 24);
    // 30日で完全に白黒になる計算 (適宜調整可能)
    const saturation = Math.max(0, 100 - diffDays * 3.33);
    return { filter: `grayscale(${100 - saturation}%)` };
  };

  return (
    <div className="container">
      <h1>DreamAnchor</h1>

      <div className="add-form">
        <input
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          placeholder="やりたいこと..."
        />
        <select value={category} onChange={(e) => setCategory(e.target.value)}>
          <option>旅・冒険</option>
          <option>スキル・成長</option>
          <option>ライフスタイル・体験</option>
        </select>
        <textarea
          value={futureMessage}
          onChange={(e) => setFutureMessage(e.target.value)}
          placeholder="未来の自分へのメッセージ"
        />
        <button onClick={addItem}>追加する</button>
      </div>

      <div className="list">
        {items.map((item) => (
          <div
            key={item.id}
            className={`item ${item.status}`}
            style={item.status === "active" ? calculateAging(item.last_touched_at) : {}}
          >
            <div className="item-content" onClick={() => item.status === "active" && touchItem(item.id)}>
              <h3>{item.title}</h3>
              <span>{item.category}</span>
            </div>
            {item.status === "active" && (
              <button className="complete-btn" onClick={() => completeItem(item)}>達成！</button>
            )}
          </div>
        ))}
      </div>

      {showPopup && (
        <div className="modal" onClick={() => setShowPopup(null)}>
          <div className="modal-content">
            <h2>おめでとう！達成しました！</h2>
            <p>過去のあなたからのメッセージ：</p>
            <div className="message-box">{showPopup}</div>
            <button onClick={() => setShowPopup(null)}>閉じる</button>
          </div>
        </div>
      )}
    </div>
  );
}

export default App;
