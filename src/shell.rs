pub fn generate_init_script(shell: &str) -> Option<String> {
    let te_path = get_te_path();
    match shell {
        "zsh" => Some(generate_zsh_script(&te_path)),
        "bash" => Some(generate_bash_script(&te_path)),
        "fish" => Some(generate_fish_script(&te_path)),
        _ => None,
    }
}

fn get_te_path() -> String {
    // Get the path to the current executable
    if let Ok(exe_path) = std::env::current_exe() {
        // If running from target/debug or target/release, use the full path
        if let Some(path_str) = exe_path.to_str() {
            if path_str.contains("/target/debug/") || path_str.contains("/target/release/") {
                return path_str.to_string();
            }
        }
    }
    // Otherwise, just use "te" (assumes it's in PATH)
    "te".to_string()
}

fn generate_zsh_script(te_path: &str) -> String {
    format!(r#"# te shell integration for zsh

# Function to run te and execute the resulting command
te-run() {{
    local result
    result=$({} "$@")
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        eval "$result"
    fi
}}

# Widget to invoke te with current buffer content
te-widget() {{
    if [ -n "$BUFFER" ]; then
        local result
        result=$({} $BUFFER)
        local ret=$?
        zle reset-prompt
        if [ $ret -eq 0 ] && [ -n "$result" ]; then
            BUFFER="$result"
            zle end-of-line
        fi
        return $ret
    fi
}}

# Register the widget
zle -N te-widget

# Bind Ctrl+T to the widget (you can customize this)
bindkey '^T' te-widget
"#, te_path, te_path)
}

fn generate_bash_script(te_path: &str) -> String {
    format!(r#"# te shell integration for bash

# Function to run te and execute the resulting command
te-run() {{
    local result
    result=$({} "$@")
    if [ $? -eq 0 ] && [ -n "$result" ]; then
        eval "$result"
    fi
}}
"#, te_path)
}

fn generate_fish_script(te_path: &str) -> String {
    format!(r#"# te shell integration for fish

# Function to run te and execute the resulting command
function te-run
    set -l result ({} $argv)
    if test $status -eq 0 -a -n "$result"
        eval $result
    end
end
"#, te_path)
}
