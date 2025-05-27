# Perseus

**Advanced Network Scanning & Vulnerability Management Platform**

---

## 🚀 Features

✅ High-speed, distributed scanning (Nmap, Nikto, SSLyze)  
✅ Plugin management system  
✅ Multi-worker architecture for scaling  
✅ WebSocket live scan updates  
✅ Admin dashboard + RBAC (role-based access control)  
✅ Report export (PDF, CSV, JSON)  
✅ Audit logging of sensitive actions  
✅ Scheduler for updates + periodic tasks  
✅ REST API + WebSocket API  
✅ Postgres database backend  
✅ Docker Compose deployment ready

---

## 🔒 Privacy & Security

- Perseus runs **entirely on your infrastructure**; no external data sharing.  
- Scan targets, results, logs, and reports remain under your control.  
- Strong authentication + RBAC system protects admin functions.  
- Plugin executions are sandboxed (but review carefully before adding custom plugins).  
- Audit logs track who did what, when, and on which targets.

---

## ⚠️ No Liability Disclaimer

This software is provided **as is**, without warranty of any kind, express or implied.  
The authors and contributors assume **no responsibility or liability** for damages, losses, or legal issues arising from the use or misuse of this software.  
It is **your responsibility** to ensure you have proper authorization before conducting any network scans or security tests.

---

## 🏗️ How to Deploy

### 1️⃣ Prerequisites
- Docker + Docker Compose installed  
- `.env` file configured (see `.env.example`)  
- Optional: Postgres admin access if you want to inspect DB directly

### 2️⃣ Deploy
```bash
docker compose up --build
```

### 3️⃣ Access
- Frontend: http://localhost:3000  
- Backend API: http://localhost:8000

### 4️⃣ Logins
- Create admin and user accounts via API or seed script (RBAC enforced).

---

## 📋 Important Notes

- Only use Perseus on networks you own or have explicit authorization to test.  
- Regularly update plugins, Nmap scripts, and dependencies (`cargo audit`, `npm audit`).  
- Consider migrating to Kubernetes + Helm for cloud-scale deploys (scaffold included).  
- Add Prometheus/Grafana monitoring for production observability.

---

## 📄 License

MIT License

---

For questions or security concerns, refer to the included `SECURITY.md`.
