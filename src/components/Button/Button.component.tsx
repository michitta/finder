import { FC, PropsWithChildren } from "react";
import styles from "./Button.module.scss";
import { spawnLayout } from "../../utils/windowManager";

const Button: FC<PropsWithChildren> = ({ children }) => {
    return (
        <button className={styles.button} onClick={() => spawnLayout()}>
            {children}
        </button >
    )
}

export default Button;