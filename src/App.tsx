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
      const intl = new Intl.DateTimeFormat('ru-RU', { timeStyle: 'short', dateStyle: 'full' });
      setTime(intl.format(date));
    }, 60000)

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
        <h1>{windowName}</h1>
        <div className={styles.contextMenu}>
          <Button>File</Button>
          <Button>Edit</Button>
          <Button>View</Button>
          <Button>Go</Button>
          <Button>Commands</Button>
          <Button>Window</Button>
          <Button>Help</Button>
        </div>
      </section>
      <section className={styles.rightSection}>
        <p>{time}</p>
      </section>
    </div>
  );
}

export default App;
