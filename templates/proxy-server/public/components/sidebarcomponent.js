class SidebarComponent extends HTMLElement {
    connectedCallback() {
        this.innerHTML = `
            <aside class="w-64 bg-gray-800 text-white h-screen">
                <div class="p-4 text-center">
                    <h2 class="text-2xl font-semibold">OCFS</h2>
                </div>
                <nav>
                    <ul>
                        <li>
                            <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700" onclick="toggleMenu('dashboard-menu')">
                                <i class="fas fa-tachometer-alt mr-2"></i> Dashboard
                            </button>
                            <ul id="dashboard-menu" class="hidden-submenu pl-8">
                                <li class="sublink py-1"><a href="dashboard/overview"><i class="fas fa-chart-line mr-2"></i> Overview</a></li>
                                <li class="sublink py-1"><a href="dashboard/statistics"><i class="fas fa-chart-bar mr-2"></i> Statistics</a></li>
                            </ul>
                        </li>
                        <li>
                            <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700" onclick="toggleMenu('cases-menu')">
                                <i class="fas fa-folder-open mr-2"></i> Cases
                            </button>
                            <ul id="cases-menu" class="hidden-submenu pl-8">
                                <li class="sublink py-1"><a href="cases/all"><i class="fas fa-folder mr-2"></i> All Cases</a></li>
                                <li class="sublink py-1"><a href="cases/new"><i class="fas fa-plus mr-2"></i> Create New Case</a></li>
                                <li class="sublink py-1"><a href="cases/types"><i class="fas fa-list-alt mr-2"></i> Case Types</a></li>
                            </ul>
                        </li>
                        <li>
                            <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700" onclick="toggleMenu('documents-menu')">
                                <i class="fas fa-file-alt mr-2"></i> Documents
                            </button>
                            <ul id="documents-menu" class="hidden-submenu pl-8">
                                <li class="sublink py-1"><a href="documents/all"><i class="fas fa-file mr-2"></i> All Documents</a></li>
                                <li class="sublink py-1"><a href="documents/upload"><i class="fas fa-upload mr-2"></i> Upload Document</a></li>
                                <li class="sublink py-1"><a href="documents/types"><i class="fas fa-file-alt mr-2"></i> Document Types</a></li>
                            </ul>
                        </li>
                        <li>
                            <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700" onclick="toggleMenu('scheduling-menu')">
                                <i class="fas fa-calendar-alt mr-2"></i> Scheduling
                            </button>
                            <ul id="scheduling-menu" class="hidden-submenu pl-8">
                                <li class="sublink py-1"><a href="scheduling/calendar"><i class="fas fa-calendar mr-2"></i> Calendar</a></li>
                                <li class="sublink py-1"><a href="scheduling/deadlines"><i class="fas fa-hourglass-half mr-2"></i> Deadlines</a></li>
                                <li class="sublink py-1"><a href="scheduling/reminders"><i class="fas fa-bell mr-2"></i> Reminders</a></li>
                            </ul>
                        </li>
                        <li>
                            <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700" onclick="toggleMenu('reports-menu')">
                                <i class="fas fa-chart-pie mr-2"></i> Reports & Analytics
                            </button>
                            <ul id="reports-menu" class="hidden-submenu pl-8">
                                <li class="sublink py-1"><a href="reports/generate"><i class="fas fa-file-alt mr-2"></i> Generate Report</a></li>
                                <li class="sublink py-1"><a href="reports/analytics"><i class="fas fa-chart-line mr-2"></i> View Analytics</a></li>
                            </ul>
                        </li>
                        <li>
                            <button class="w-full text-left px-4 py-2 hover:bg-gray-700 focus:outline-none focus:bg-gray-700" onclick="toggleMenu('user-management-menu')">
                                <i class="fas fa-users-cog mr-2"></i> User Management
                            </button>
                            <ul id="user-management-menu" class="hidden-submenu pl-8">
                                <li class="sublink py-1"><a href="user-management/users"><i class="fas fa-users mr-2"></i> All Users</a></li>
                                <li class="sublink py-1"><a href="user-management/new"><i class="fas fa-user-plus mr-2"></i> Add New User</a></li>
                                <li class="sublink py-1"><a href="user-management/roles"><i class="fas fa-user-shield mr-2"></i> Roles & Permissions</a></li>
                            </ul>
                        </li>
                    </ul>
                </nav>
            </aside>
        `;
    }
}
customElements.define('sidebar-component', SidebarComponent);
