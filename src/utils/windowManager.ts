import { LogicalPosition, Window } from "@tauri-apps/api/window";

export function spawnLayout() {
  const layoutWindow = new Window("layout");

  layoutWindow.isVisible().then((visible) => {
    console.log(visible);
    if (visible) {
      layoutWindow.hide();
    } else {
      layoutWindow.show();
      layoutWindow.setPosition(new LogicalPosition(0, 0));
    }
  });
}
