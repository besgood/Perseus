import { useEffect, useState } from 'react';
import axios from 'axios';

export default function Dashboard() {
    const [status, setStatus] = useState('');

    useEffect(() => {
        axios.get('/api/status').then(res => setStatus(res.data.status));
    }, []);

    const startScan = async () => {
        const res = await axios.post('/api/scan/start');
        alert(res.data.message);
    };

    return (
        <div className="p-4">
            <h1 className="text-2xl">Dashboard</h1>
            <p>Status: {status}</p>
            <button onClick={startScan} className="mt-2 px-4 py-2 bg-blue-500 text-white rounded">Start Scan</button>
        </div>
    );
}
