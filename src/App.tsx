import { useEffect, useState } from "react";
import styles from "./App.module.scss";
import Button from "./components/Button/Button.component";
import { listen } from "@tauri-apps/api/event";

function App() {
  const [time, setTime] = useState<string | undefined>();
  const [windowName, setWindowName] = useState<string | undefined>("Finder");

  useEffect(() => {
    const interval = setInterval(() => {
      const date = new Date();
      const intl = new Intl.DateTimeFormat('ru-RU', { timeStyle: 'short', dateStyle: 'short' });
      setTime(intl.format(date));
    }, 2000)

    return () => clearInterval(interval)
  }, [])

  useEffect(() => {
    const windowName = listen<string>("window-name", (e) => {
      if (e.payload) setWindowName(e.payload);
    });

    return () => {
      windowName.then((f) => f())
    };
  }, [])

  return (
    <div className={styles.finder}>
      <section className={styles.leftSection}>
        <Button>🐱</Button>
        <h1>{windowName}</h1>
        <div className={styles.contextMenu}>
          <Button>Файл</Button>
          <Button>Правка</Button>
          <Button>Вид</Button>
          <Button>Перейти</Button>
          <Button>Комманды</Button>
          <Button>Окно</Button>
          <Button>Помощь</Button>
        </div>
      </section>
      <section className={styles.rightSection}>
        <p>{time}</p>
      </section>
    </div>
  );
}

export default App;
