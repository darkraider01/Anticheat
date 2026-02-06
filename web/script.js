document.addEventListener('DOMContentLoaded', () => {
    const sidebarToggle = document.getElementById('sidebarToggle');
    const sidebar = document.querySelector('.sidebar');
    const wrapper = document.querySelector('.wrapper');

    if (sidebarToggle && sidebar && wrapper) {
        sidebarToggle.addEventListener('click', () => {
            sidebar.classList.toggle('collapsed');
            wrapper.classList.toggle('sidebar-open');
            document.querySelector('.main-content').classList.toggle('shifted');
        });
    }



    // Dashboard Logic
    // Page Routing Logic
    const path = window.location.pathname;

    if (path.includes('index.html') || path === '/') {
        startPolling();
    } else if (path.includes('agents.html')) {
        fetchAgents();
    } else if (path.includes('alerts.html')) {
        fetchAlerts();
    }

    // --- Dashboard (Live Detections) ---
    function startPolling() {
        fetchDetections();
        setInterval(fetchDetections, 5000); // Poll every 5 seconds
    }

    async function fetchDetections() {
        try {
            const response = await fetch('/v1/detections');
            
            if (response.status === 401) {
                window.location.href = '/login.html';
                return;
            }

            if (response.ok) {
                const data = await response.json();
                renderDetections(data.data);
            }
        } catch (error) {
            console.error('Failed to fetch detections:', error);
        }
    }

    function renderDetections(detections) {
        const container = document.querySelector('.live-detections');
        if (!detections || detections.length === 0) {
            container.innerHTML = `
                <h2>Live Detections</h2>
                <div class="empty-state">
                    <p>No live detections at the moment.</p>
                    <p>Monitor your game servers for suspicious activity.</p>
                </div>`;
            return;
        }

        const listHtml = detections.map(d => `
            <div class="detection-card ${d.severity}">
                <div class="detection-header">
                    <span class="badg ${d.severity}">${d.severity.toUpperCase()}</span>
                    <span class="time">${new Date(d.created_at).toLocaleTimeString()}</span>
                </div>
                <h3>${d.title}</h3>
                <p>${d.description}</p>
                <div class="meta">
                    <small>Agent: ${d.agent_id} | Type: ${d.detection_type}</small>
                </div>
            </div>
        `).join('');

        container.innerHTML = `<h2>Live Detections</h2><div class="detection-list">${listHtml}</div>`;
    }


    // --- Agents Page ---
    async function fetchAgents() {
        try {
            const response = await fetch('/v1/agents');
            if (response.status === 401) return window.location.href = '/login.html';
            
            if (response.ok) {
                const data = await response.json();
                renderAgents(data.data);
            }
        } catch (error) { console.error(error); }
    }

    function renderAgents(agents) {
        const container = document.querySelector('.agents-list');
        if (!agents || agents.length === 0) return container.innerHTML = '<p>No agents found.</p>';

        const tableRows = agents.map(a => `
            <tr>
                <td>${a.name}</td>
                <td>${a.platform}</td>
                <td>${a.version}</td>
                <td><span class="badg ${a.status === 'online' ? 'low' : 'high'}">${a.status}</span></td>
                <td>${new Date(a.last_heartbeat).toLocaleString()}</td>
            </tr>
        `).join('');

        container.innerHTML = `
            <h2>Active Agents</h2>
            <table class="data-table">
                <thead><tr><th>Name</th><th>Platform</th><th>Version</th><th>Status</th><th>Last Heartbeat</th></tr></thead>
                <tbody>${tableRows}</tbody>
            </table>
        `;
    }

    // --- Alerts Page ---
    async function fetchAlerts() {
         try {
            const response = await fetch('/v1/alerts');
            if (response.status === 401) return window.location.href = '/login.html';
            
            if (response.ok) {
                const data = await response.json();
                renderAlerts(data.data);
            }
        } catch (error) { console.error(error); }
    }

    function renderAlerts(alerts) {
        const container = document.querySelector('.alerts-list');
        if (!alerts || alerts.length === 0) return container.innerHTML = '<p>No alerts found.</p>';

        const listHtml = alerts.map(a => `
             <div class="detection-card ${a.severity}">
                <div class="detection-header">
                    <span class="badg ${a.severity}">${a.severity.toUpperCase()}</span>
                    <span class="time">${new Date(a.created_at).toLocaleString()}</span>
                </div>
                <h3>${a.title}</h3>
                <p>${a.description}</p>
                 <div class="meta">
                    <small>Rule: ${a.rule_id} | Status: ${a.status}</small>
                </div>
            </div>
        `).join('');
        
        container.innerHTML = `<h2>Recent Alerts</h2><div class="detection-list">${listHtml}</div>`;
    }
});