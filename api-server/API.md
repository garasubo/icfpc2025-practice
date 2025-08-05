# API Documentation

## Spaceship Resources API

### GET /api/spaceship/{filename}

Spaceshipリソースディレクトリ内のtxtファイルの内容を取得します。

#### パラメータ

- `filename` (path parameter, required): ファイル名（拡張子なし）
  - 例: `problem1`, `problem25`
  - 英数字とハイフンのみ許可（セキュリティ対策）

#### レスポンス

**成功時 (200 OK):**
```json
{
  "success": true,
  "data": {
    "filename": "problem1",
    "content": "1 -1\n1 -3\n2 -5\n2 -8\n3 -10\n\n"
  },
  "message": "File retrieved successfully"
}
```

**ファイルが存在しない場合 (404 Not Found):**
```
HTTP 404 Not Found
```

**不正なファイル名の場合 (400 Bad Request):**
```
HTTP 400 Bad Request
```

**サーバーエラー (500 Internal Server Error):**
```
HTTP 500 Internal Server Error
```
