import styles from "./Dock.module.scss";

function Dock() {
    return (
        <div className={styles.dock}>
            <div className={styles.element} />
            <div className={styles.element} />
            <div className={styles.element} />
            <div className={styles.element} />
            <div className={styles.element} />
        </div>
    );
}

export default Dock;
