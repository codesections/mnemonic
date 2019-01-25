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
            
            add)
                cmd+="__add"
                ;;
            edit)
                cmd+="__edit"
                ;;
            help)
                cmd+="__help"
                ;;
            list)
                cmd+="__list"
                ;;
            rm)
                cmd+="__rm"
                ;;
            show)
                cmd+="__show"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        mn)
            opts=" -p -h -V  --plaintext --help --version  <MNEMONIC>  add edit list rm show help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        
        mn__add)
            opts=" -b -h -V  --blank --help --version  <MNEMONIC> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        mn__edit)
            opts=" -h -V -p  --help --version --push  <MNEMONIC> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
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
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        mn__help)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        mn__list)
            opts=" -h -V  --help --version  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        mn__rm)
            opts=" -f -h -V  --force --help --version  <MNEMONIC>... "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
            return 0
            ;;
        mn__show)
            opts=" -p -h -V -t -s  --plaintext --help --version --theme --syntax  <MNEMONIC> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
                return 0
            fi
            case "${prev}" in
                
                --theme)
                    COMPREPLY=($(compgen -W "1337 DarkNeon GitHub Monokai Extended Monokai Extended Bright Monokai Extended Light Monokai Extended Origin OneHalfDark OneHalfLight Sublime Snazzy TwoDark zenburn" -- ${cur}))
                    return 0
                    ;;
                    -t)
                    COMPREPLY=($(compgen -W "1337 DarkNeon GitHub Monokai Extended Monokai Extended Bright Monokai Extended Light Monokai Extended Origin OneHalfDark OneHalfLight Sublime Snazzy TwoDark zenburn" -- ${cur}))
                    return 0
                    ;;
                --syntax)
                    COMPREPLY=($(compgen -f ${cur}))
                    return 0
                    ;;
                    -s)
                    COMPREPLY=($(compgen -f ${cur}))
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
