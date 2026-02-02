# 🦀 Rust Axum Messaging API

Bu proje, **Rust** programlama dili ve **Axum** web framework'ü kullanılarak geliştirilmiş, ölçeklenebilir ve modern bir mesajlaşma API'sidir. Proje, temiz kod prensiplerine uygun olarak **Katmanlı Mimari (Layered Architecture)** yapısında tasarlanmıştır.

## 🚀 Teknolojiler

Projede kullanılan temel kütüphaneler ve araçlar:

- **Dil:** Rust  
- **Web Framework:** Axum (Ergonomik ve modüler)  
- **Veritabanı Sürücüsü & ORM:** SQLx (Derleme zamanı SQL kontrolü)  
- **Veritabanı:** PostgreSQL  
- **Asenkron Runtime:** Tokio  
- **Serializasyon:** Serde & Serde JSON  

## 📂 Proje Mimarisi

Kod tabanı, sorumlulukların ayrılması ilkesine (Separation of Concerns) göre klasörlenmiştir:

- `src/api` → Controller / Handler katmanı  
- `src/domain` → Model katmanı (Struct’lar, domain objeleri)  
- `src/repo` → Data Access katmanı (SQL sorguları)  
- `src/infra` → Altyapı (config, error handling)  
- `src/app.rs` → Router ve Application State  

## 🛠 Kurulum ve Çalıştırma

### 1. Gereksinimler
- Rust (Cargo yüklü olmalı)
- PostgreSQL
- SQLx CLI  
```bash
cargo install sqlx-cli
```

### 2. Ortam Değişkenleri (.env)
Proje kök dizininde `.env` dosyası oluşturun:

```env
DATABASE_URL=postgres://kullanici_adi:sifre@localhost:5432/veritabani_adi
```

### 3. Veritabanı Hazırlığı

```bash
# Veritabanı yoksa oluştur
sqlx database create

# Migration’ları çalıştır
sqlx migrate run
```

### 4. Uygulamayı Başlatma

```bash
cargo run
```

Sunucu şu adreste çalışır:  
http://127.0.0.1:8000

## 📡 API Dokümantasyonu

### 👤 Kullanıcı İşlemleri (User API)

| Metot | Endpoint | Açıklama |
|------|---------|----------|
| POST | /users | Yeni kullanıcı oluşturur |
| GET | /users | Tüm kullanıcıları listeler |
| GET | /users/{id} | Tekil kullanıcı getirir |

**Örnek JSON (Kullanıcı Oluşturma):**
```json
{
  "username": "yazilimci_arkadas"
}
```

### 💬 Mesajlaşma İşlemleri (Message API)

| Metot | Endpoint | Açıklama |
|------|---------|----------|
| POST | /messages | Mesaj gönder |
| GET | /users/{id}/inbox | Gelen kutusu |
| GET | /users/{id}/sent | Giden kutusu |

**Örnek JSON (Mesaj Gönderme):**
```json
{
  "from_user_id": "gonderen-uuid",
  "to_user_id": "alici-uuid",
  "content": "Selam, Rust ile backend geliştirmek harika!"
}
```

## 🧪 Test Etme

```bash
curl -X POST http://127.0.0.1:8000/messages \
-H "Content-Type: application/json" \
-d '{"from_user_id":"...","to_user_id":"...","content":"Merhaba Dünya"}'
```
