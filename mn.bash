_mn() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            mn)
                cmd="mn"
                ;;
            
            *)
                ;;
        esac
    done

    case "${cmd}" in
        mn)
            opts=" -a -n -l -r -e -h -V -p -t  --add --new --list --rm --edit --help --version --push --theme  <MNEMONIC> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --push)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                --theme)
                    COMPREPLY=($(compgen -W "1337 DarkNeon GitHub Monokai Extended Monokai Extended Bright Monokai Extended Light Monokai Extended Origin OneHalfDark OneHalfLight Sublime Snazzy TwoDark zenburn" -- ${cur}))
                    return 0
                    ;;
                    -t)
                    COMPREPLY=($(compgen -W "1337 DarkNeon GitHub Monokai Extended Monokai Extended Bright Monokai Extended Light Monokai Extended Origin OneHalfDark OneHalfLight Sublime Snazzy TwoDark zenburn" -- ${cur}))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
    esac
}

complete -F _mn -o bashdefault -o default mn
