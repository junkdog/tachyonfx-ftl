// custom completer for tachyonfx effect dsl
const tfxCustomCompleter = {
    getCompletions: function(editor, session, pos, prefix, callback) {
        // Call into Rust for completions
        if (window.wasmBindings && window.wasmBindings.get_completions) {
            try {
                const source = session.getValue();
                const line = pos.row;
                const column = pos.column;

                const completionsJson = window.wasmBindings.get_completions(source, line, column);
                const completions = JSON.parse(completionsJson);

                callback(null, completions);
            } catch (error) {
                console.error("Error getting completions from Rust:", error);
                // Fallback to empty completions on error
                callback(null, []);
            }
        } else {
            // Fallback if WASM bindings not ready yet
            callback(null, []);
        }
    }
};

// export the completer so it can be imported in other files
// export default tfxCustomCompleter;