class ExamplePanel {
    constructor() {
        this.examples = [];
        this.filteredExamples = [];
        this.currentCategory = 'all';
        this.currentExample = null;
        this.searchTerm = '';

        // Wait for WASM to be ready before loading
        this.initializeWhenReady();
    }

    initializeWhenReady() {
        // Check if WASM bindings are available
        if (window.wasmBindings && window.wasmBindings.get_example_list) {
            this.loadExamplesList();
        } else {
            // Retry after a short delay
            setTimeout(() => this.initializeWhenReady(), 100);
        }
    }

    async loadExamplesList() {
        try {
            // Call Rust function to get all available examples
            const listJson = window.wasmBindings.get_example_list();
            this.examples = JSON.parse(listJson);
            this.filteredExamples = [...this.examples];

            this.populateCategoryFilter();
            this.renderExamplesPanel();
            this.bindEventListeners();

            console.log(`Loaded ${this.examples.length} examples from Rust backend`);
        } catch (error) {
            console.error('Failed to load examples from Rust backend:', error);
            this.renderErrorState();
        }
    }

    populateCategoryFilter() {
        const categoryFilter = document.getElementById('category-filter');
        if (!categoryFilter) return;

        // Get unique categories
        const categories = [...new Set(this.examples.map(ex => ex.category))];

        // Clear existing options except "All Categories"
        categoryFilter.innerHTML = '<option value="all">All Categories</option>';

        // Add category options
        categories.forEach(category => {
            const option = document.createElement('option');
            option.value = category.toLowerCase();
            option.textContent = category;
            categoryFilter.appendChild(option);
        });
    }

    renderExamplesPanel() {
        const examplesList = document.getElementById('examples-list');
        if (!examplesList) return;

        if (this.filteredExamples.length === 0) {
            examplesList.innerHTML = '<div class="loading-state">No examples found</div>';
            return;
        }

        // Group examples by category
        const categorizedExamples = this.groupByCategory(this.filteredExamples);

        examplesList.innerHTML = Object.entries(categorizedExamples)
            .map(([category, examples]) => this.renderCategory(category, examples))
            .join('');
    }

    groupByCategory(examples) {
        return examples.reduce((groups, example) => {
            const category = example.category;
            if (!groups[category]) {
                groups[category] = [];
            }
            groups[category].push(example);
            return groups;
        }, {});
    }

    renderCategory(category, examples) {
        return `
            <div class="example-category" data-category="${category}">
                <h4 class="category-title">${category}</h4>
                <div class="category-examples">
                    ${examples.map(example => this.renderExample(example)).join('')}
                </div>
            </div>
        `;
    }

    renderExample(example) {
        const isActive = this.currentExample === example.id ? 'active' : '';
        return `
            <div class="example-item ${isActive}" data-example-id="${example.id}">
                <div class="example-title">${example.title}</div>
                <div class="example-description">${example.description}</div>
            </div>
        `;
    }

    renderErrorState() {
        const examplesList = document.getElementById('examples-list');
        if (examplesList) {
            examplesList.innerHTML = `
                <div class="loading-state" style="color: #fb4934;">
                    Failed to load examples. Please refresh the page.
                </div>
            `;
        }
    }

    bindEventListeners() {
        // Example item clicks
        const examplesList = document.getElementById('examples-list');
        if (examplesList) {
            examplesList.addEventListener('click', (e) => {
                const exampleItem = e.target.closest('.example-item');
                if (exampleItem) {
                    const exampleId = exampleItem.dataset.exampleId;
                    this.loadExample(exampleId);
                }
            });
        }

        // Search input
        const searchInput = document.getElementById('example-search');
        if (searchInput) {
            searchInput.addEventListener('input', (e) => {
                this.searchTerm = e.target.value.toLowerCase();
                this.filterExamples();
            });
        }

        // Category filter
        const categoryFilter = document.getElementById('category-filter');
        if (categoryFilter) {
            categoryFilter.addEventListener('change', (e) => {
                this.currentCategory = e.target.value;
                this.filterExamples();
            });
        }

        // Panel collapse toggle
        const collapseBtn = document.getElementById('examples-collapse-btn');
        if (collapseBtn) {
            collapseBtn.addEventListener('click', () => {
                this.togglePanel();
            });
        }
    }

    filterExamples() {
        this.filteredExamples = this.examples.filter(example => {
            // Category filter
            const categoryMatch = this.currentCategory === 'all' ||
                                  example.category.toLowerCase() === this.currentCategory;

            // Search filter
            const searchMatch = this.searchTerm === '' ||
                                example.title.toLowerCase().includes(this.searchTerm) ||
                                example.description.toLowerCase().includes(this.searchTerm);

            return categoryMatch && searchMatch;
        });

        this.renderExamplesPanel();
    }

    async loadExample(exampleId) {
        try {
            // Clear any existing error markers
            if (window.clearErrorMarkers) {
                window.clearErrorMarkers();
            }

            // Call Rust function to load the example
            const resultJson = window.wasmBindings.load_example(exampleId);
            const result = JSON.parse(resultJson);

            if (result.error) {
                console.error('Example load error:', result.error);
                this.showError(`Failed to load example: ${result.error}`);
                return;
            }

            // Update current example tracking
            this.currentExample = exampleId;

            // Update URL parameters (reuse existing query-params functions)
            updateQueryParam('code', result.code);
            updateQueryParam('canvas', result.canvas);
            updateQueryParam('example', exampleId);

            // Update editor and canvas (reuse existing functions)
            const editor = window.editorInstance || window.editor; // Try both names
            const canvasInput = document.getElementById('canvas-input');

            if (editor && canvasInput) {
                // Decompress and set the content
                const code = decodeAndInflate(result.code);
                const canvas = decodeAndInflate(result.canvas);

                editor.setValue(code, -1);
                canvasInput.value = canvas;

                // Update UI to show active example
                this.updateActiveExample(exampleId);

                // Trigger compilation
                if (window.wasmBindings.compile_dsl && window.wasmBindings.update_canvas) {
                    window.wasmBindings.update_canvas(canvas);
                    window.wasmBindings.compile_dsl(code);
                }

                console.log(`Loaded example: ${result.title}`);
            } else {
                console.error('Editor or canvas input not found');
            }
        } catch (error) {
            console.error('Failed to load example:', error);
            this.showError('Internal error loading example');
        }
    }

    updateActiveExample(exampleId) {
        // Remove active class from all items
        document.querySelectorAll('.example-item').forEach(item => {
            item.classList.remove('active');
        });

        // Add active class to selected item
        const activeItem = document.querySelector(`[data-example-id="${exampleId}"]`);
        if (activeItem) {
            activeItem.classList.add('active');
        }
    }

    togglePanel() {
        const panel = document.getElementById('examples-panel');
        const button = document.getElementById('examples-collapse-btn');

        if (panel && button) {
            const isCollapsed = panel.classList.contains('collapsed');

            if (isCollapsed) {
                panel.classList.remove('collapsed');
                button.textContent = '←';
                button.title = 'Collapse examples panel';
            } else {
                panel.classList.add('collapsed');
                button.textContent = '→';
                button.title = 'Expand examples panel';
            }
        }
    }

    showError(message) {
        // Integrate with existing error toast system if available
        if (window.showError) {
            window.showError(message);
        } else {
            // Fallback to console
            console.error('Example Panel Error:', message);
        }
    }

    // Public method to load example by ID (for external use)
    loadExampleById(exampleId) {
        const example = this.examples.find(ex => ex.id === exampleId);
        if (example) {
            this.loadExample(exampleId);
            return true;
        }
        return false;
    }

    // Public method to get all available examples
    getAvailableExamples() {
        return this.examples.map(ex => ({
            id: ex.id,
            title: ex.title,
            category: ex.category,
            description: ex.description
        }));
    }
}

// Export for global access
window.ExamplePanel = ExamplePanel;