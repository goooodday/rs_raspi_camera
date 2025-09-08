# 🎥 라즈베리 파이 카메라 스트림

라즈베리 파이에서 실시간 카메라 영상을 웹브라우저로 스트리밍하는 Rust 애플리케이션입니다.

## ✨ 기능

- **실시간 비디오 스트리밍**: 라즈베리 파이 카메라의 영상을 실시간으로 웹브라우저에 전송
- **WebSocket 통신**: 낮은 지연시간으로 고품질 스트리밍
- **반응형 웹 인터페이스**: 모바일과 데스크톱 모두 지원
- **성능 모니터링**: FPS, 지연시간, 데이터 사용량 등 실시간 통계
- **자동 재연결**: 연결이 끊어져도 자동으로 재연결 시도
- **전체화면 지원**: 풀스크린 모드로 영상 시청 가능

## 🚀 시작하기 (라즈베리 파이에서 직접 빌드 - 권장)

이 방법은 라즈베리 파이에서 직접 소스 코드를 컴파일하는 가장 간단하고 확실한 방법입니다.

### 1. 라즈베리 파이 설정

```bash
# 시스템 업데이트
sudo apt update && sudo apt upgrade -y

# 카메라 모듈 활성화 (raspi-config 사용)
sudo raspi-config
# -> 3 Interface Options -> I1 Camera -> Yes 선택 후 재부팅

# 빌드에 필요한 패키지 설치
sudo apt install -y build-essential pkg-config libopencv-dev libclang-dev v4l-utils
```

### 2. Rust 설치

```bash
# 라즈베리 파이 터미널에서 실행
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### 3. 프로젝트 빌드 및 실행

```bash
# GitHub에서 소스 코드 클론
git clone https://github.com/<your-github-username>/raspberry_pi_camera_stream.git
cd raspberry_pi_camera_stream

# 프로젝트 빌드 (최적화된 릴리스 버전)
cargo build --release

# 실행
./target/release/raspberry_pi_camera_stream
```

### 4. 웹브라우저에서 접속

다른 컴퓨터나 스마트폰의 웹 브라우저에서 `http://<라즈베리파이_IP>:3030` 주소로 접속하세요.

---

## 🔧 고급 사용법: 크로스 컴파일

개발용 PC(macOS, Linux)에서 라즈베리 파이용 실행 파일을 미리 컴파일하는 방법입니다. 빌드 속도가 빠르다는 장점이 있습니다.

### 1. 크로스 컴파일 환경 설정 (PC에서)

```bash
# Rust 타겟 추가 (64비트 OS 기준)
rustup target add aarch64-unknown-linux-gnu

# 크로스 컴파일러 설치
# macOS의 경우
brew install aarch64-linux-gnu

# Ubuntu/Debian의 경우
sudo apt install gcc-aarch64-linux-gnu
```

### 2. Cargo 설정 (PC에서)

프로젝트 루트에 `.cargo/config.toml` 파일을 생성하고 아래 내용을 추가합니다.

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

### 3. 크로스 컴파일 실행 (PC에서)

```bash
# aarch64 타겟으로 빌드
# 이 방법은 PC에 OpenCV 라이브러리가 설치되어 있어야 할 수 있습니다.
cargo build --release --target=aarch64-unknown-linux-gnu
```

### 4. 파일 전송 및 실행

```bash
# scp를 이용해 컴파일된 바이너리를 라즈베리 파이로 전송
scp target/aarch64-unknown-linux-gnu/release/raspberry_pi_camera_stream pi@<라즈베리파이_IP>:~

# 라즈베리 파이에 SSH로 접속하여 실행
ssh pi@<라즈베리파이_IP>
chmod +x ./raspberry_pi_camera_stream
./raspberry_pi_camera_stream
```

---

## ⚙️ 구성 옵션

환경 변수를 통해 애플리케이션을 구성할 수 있습니다.

- `RUST_LOG`: 로그 레벨 설정 ( `debug`, `info`, `warn`, `error` )
- 서버 포트 및 카메라 해상도 등은 `src/main.rs` 파일에서 직접 수정할 수 있습니다.

## 🛠️ 문제 해결

### 카메라가 인식되지 않는 경우
- `sudo raspi-config`를 통해 카메라가 활성화되었는지 다시 확인하고 재부팅하세요.
- `sudo usermod -a -G video $USER` 명령어로 현재 사용자를 `video` 그룹에 추가한 후 재로그인하세요.

### 네트워크 접속 문제
- `hostname -I` 명령어로 라즈베리 파이의 IP 주소를 확인하세요.
- `sudo ufw status`로 방화벽 설정을 확인하고, `sudo ufw allow 3030`으로 포트를 허용해주세요.

## 🙏 감사의 말

- [OpenCV](https://opencv.org/)
- [Tokio](https://tokio.rs/)
- [Warp](https://github.com/seanmonstar/warp)
