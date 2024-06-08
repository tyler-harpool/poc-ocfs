class StatsComponent extends HTMLElement {
    connectedCallback() {
        this.innerHTML = `
            <div id="stats-section" class="hidden">
                <h2 class="text-2xl font-semibold mb-4">Quick Stats</h2>
                <div class="grid grid-cols-1 gap-4">
                    <div class="p-4 bg-white rounded-lg shadow">
                        <div class="flex items-center mb-2">
                            <i class="fas fa-briefcase text-2xl text-blue-500 mr-2"></i>
                            <h3 class="text-lg font-medium">Cases Opened</h3>
                        </div>
                        <div class="relative pt-1">
                            <div class="flex mb-2 items-center justify-between">
                                <div>
                                    <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-blue-600 bg-blue-200">75%</span>
                                </div>
                                <div class="text-right">
                                    <span class="text-xs font-semibold inline-block text-blue-600">75%</span>
                                </div>
                            </div>
                            <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-blue-200">
                                <div style="width:75%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-blue-500"></div>
                            </div>
                        </div>
                    </div>
                    <div class="p-4 bg-white rounded-lg shadow">
                        <div class="flex items-center mb-2">
                            <i class="fas fa-check text-2xl text-green-500 mr-2"></i>
                            <h3 class="text-lg font-medium">Cases Closed</h3>
                        </div>
                        <div class="relative pt-1">
                            <div class="flex mb-2 items-center justify-between">
                                <div>
                                    <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-green-600 bg-green-200">60%</span>
                                </div>
                                <div class="text-right">
                                    <span class="text-xs font-semibold inline-block text-green-600">60%</span>
                                </div>
                            </div>
                            <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-green-200">
                                <div style="width:60%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-green-500"></div>
                            </div>
                        </div>
                    </div>
                    <div class="p-4 bg-white rounded-lg shadow">
                        <div class="flex items-center mb-2">
                            <i class="fas fa-file-alt text-2xl text-yellow-500 mr-2"></i>
                            <h3 class="text-lg font-medium">Documents Processed</h3>
                        </div>
                        <div class="relative pt-1">
                            <div class="flex mb-2 items-center justify-between">
                                <div>
                                    <span class="text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full text-yellow-600 bg-yellow-200">85%</span>
                                </div>
                                <div class="text-right">
                                    <span class="text-xs font-semibold inline-block text-yellow-600">85%</span>
                                </div>
                            </div>
                            <div class="overflow-hidden h-2 mb-4 text-xs flex rounded bg-yellow-200">
                                <div style="width:85%" class="shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center bg-yellow-500"></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }
}
customElements.define('stats-component', StatsComponent);
