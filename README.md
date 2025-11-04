# blink-stm32f401

## WSL

### 📦 Установка инструментов в Windows

1. Установи `usbipd-win`:

```powershell
winget install --interactive --exact dorssel.usbipd-win
```

Перезагрузи WSL:

```powershell
wsl --shutdown
```

🔌 Подключение ST-Link к WSL:

```powershell
usbipd list
usbipd bind --busid <BUSID>    # например, 1-8
```

В обычном PowerShell (не от админа):

```powershell
usbipd attach --wsl --busid <BUSID>
```

В WSL проверь:

```bash
lsusb  # должен видеть ST-LINK
```

---

## WINDOWS

### Embedded Rust на STM32 в Windows

Этот документ описывает настройку среды для разработки на **STM32F401CCU6** в **нативной Windows**.

### 🔧 Предпосылки

- Windows 10/11
- STM32F401CCU6 (Black Pill) + ST-Link V2
- Плата запитана от USB Type-C

### 🔌 Настройка ST-Link

1. Скачай и установи **Zadig**: <https://zadig.akeo.ie/>
2. Подключи ST-Link.
3. В Zadig (от администратора):
   - Options → List All Devices
   - Выбери **ST-LINK-V2**
   - Выбери драйвер **WinUSB**
   - Нажми **Replace Driver**

### 🦀 Установка инструментов

1. Установи **Rust**: <https://rustup.rs/>
2. Добавь target:

   ```powershell
   rustup target add thumbv7em-none-eabihf
