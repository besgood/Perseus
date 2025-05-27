import { useEffect, useState } from 'react';
import axios from 'axios';

export default function Admin() {
    const [logs, setLogs] = useState([]);

    useEffect(() => {
        axios.get('/api/admin/audit').then(res => setLogs(res.data));
    }, []);

    return (
        <div className="p-4">
            <h1 className="text-2xl">Admin Audit Logs</h1>
            <ul>
                {logs.map((log, idx) => (
                    <li key={idx}>{log.action} by {log.user} on {log.timestamp}</li>
                ))}
            </ul>
        </div>
    );
}
