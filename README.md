# krtld-checker
3자리 도메인 선점을 위한 데이터마이닝 도구

본 소프트웨어는 `.env` 파일이 필요합니다. \
예제 파일:
```
KRTLD_KEY=API키 입력해주세요
KRTLD_INDEX=0
```

## 사용 방법 (Windows)
```powershell
cargo run | Tee-Object -Append -file .\output.txt
```

## 결과
https://gist.github.com/ilsubyeega/d64dbc3706da3be37170177359eae5fb