function fetchData(url) {
    return $.ajax({
        url: url,
        method: 'GET',
        dataType: 'json'
    }).fail(function(jqXHR, textStatus, errorThrown) {
        console.error("Error fetching data from", url, textStatus, errorThrown);
    });
}

function updateCpuStats(data) {
    if (!data) return;
    $("#cpu-total").text(`${data.cpu_load_average}%`);
    $("#cpu-name").text(`${data.cpu_product_name}`);
    const $coresContainer = $("#cpu-cores");
    $coresContainer.empty(); // Clear previous data
    data.cpu_usage.forEach((usage, index) => {
        const coreDiv = `
                <div class="stat-item">
                    <span class="stat-label">Core ${index}:</span>
                    <span class="stat-value">${usage.toFixed(2)}%</span>
                </div>`;
        $coresContainer.append(coreDiv);
    });
}

function updateMemoryStats(data) {
    if (!data) return;
    $("#memory-total").text(`${(data.total_memory / 1_024_000).toFixed(2)} GiB`);
    $("#memory-used").text(`${(data.used_memory / 1_024_000).toFixed(2)} GiB`);
    $("#memory-free").text(`${(data.free_memory / 1_024_000).toFixed(2)} GiB`);
    $("#memory-available").text(`${(data.available_memory / 1_024_000).toFixed(2)} GiB`);
    $("#memory-swap-total").text(`${(data.total_swap / 1_024_000).toFixed(2)} GiB`);
    $("#memory-swap-used").text(`${(data.used_swap / 1_024_000).toFixed(2)} GiB`);
    $("#memory-swap-free").text(`${(data.free_swap / 1_024_000).toFixed(2)} GiB`);
    $("#memory-percent").text(`${(data.used_memory / data.total_memory * 100).toFixed(2)}%`);
}

function updateSystemStats(data) {
    if (!data) return;
    $("#sys-name").text(data.name || "Unknown");
    $("#sys-kernel").text(data.kernel_version || "Unknown");
    $("#sys-os").text(data.os_version || "Unknown");
    $("#sys-host").text(data.host_name || "Unknown");
    $("#sys-long-os").text(data.long_os_version || "Unknown");
    $("#sys-dist").text(data.distribution_id || "Unknown");
    $("#sys-uptime").text(data.uptime || "Unknown");
}

function updateNetworkStats(data) {
    if (!data) return;
    const $networksContainer = $("#network-stats");
    $networksContainer.empty(); // Clear previous data
    data.networks.forEach((network) => {
        const networkDiv = `
                <div class="stat-item">
                    <span class="stat-label">${network.interface_name}:</span>
                    <span class="stat-value">
                        Received: ${(network.received / 1_000).toFixed(2)} KB,
                        Transmitted: ${(network.transmitted / 1_000).toFixed(2)} KB
                    </span>
                </div>`;
        $networksContainer.append(networkDiv);
    });
}

function refreshData() {
    fetchData('/cpu').done(updateCpuStats);
    fetchData('/mem').done(updateMemoryStats);
    fetchData('/system').done(updateSystemStats);
    fetchData('/networks').done(updateNetworkStats);

    // Uncomment if needed
        fetchData('/proc').done(updateProcStats);
}
function updateProcStats(processes) {
    processes = processes.processes;
    if (!processes || !Array.isArray(processes)) return;
    // processes = processes.processes;
    const $tableBody = $("#process-table tbody");
    $tableBody.empty(); // Clear existing rows

    processes.forEach((process) => {
        const rowHtml = `
            <tr>
                <td>${process.pid}</td>
                <td>${process.name}</td>
                <td>${(process.cpu_usage * 100).toFixed(2)}</td>
                <td>${(process.memory / 1_024_000).toFixed(2)}</td>
                <td>${(process.virtual_memory / 1_024_000).toFixed(2)}</td>
                <td>${process.status}</td>
                <td>${process.run_time}</td>
            </tr>
        `;
        $tableBody.append(rowHtml);
    });

}

// Set interval and initial load
setInterval(refreshData, 1000); // Replace with a reasonable interval (e.g., 10000 for 10 seconds)
refreshData();
