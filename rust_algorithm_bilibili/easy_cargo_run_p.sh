_generate_cargo_package_names() {
	local package_names=()

	while IFS= read -r file; do
		local package_name=$(grep '^name\s*=' "$file" | awk -F'"' '{print $2}')
		if [[ -n "$package_name" ]]; then
			package_names+=("$package_name")
		fi
	done < <(find . -name "Cargo.toml" 2>/dev/null) # 防止错误信息输出

	echo "${package_names[@]}"
}

_cargo_run_p_complete() {
	_arguments \
		'*:: :->args'

	case $state in
	args)
		if [[ ${words[1]} == "run" ]] && [[ ${words[2]} == "-p" ]]; then
			local packages=($(_generate_cargo_package_names))
			_wanted packages expl 'Cargo packages' compadd -a packages
		fi
		;;
	esac
}

autoload -Uz compinit && compinit
compdef _cargo_run_p_complete cargo
