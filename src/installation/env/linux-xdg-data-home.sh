#!/bin/sh
# dfxvm shell setup
# affix colons on either side of $PATH to simplify matching
case ":${PATH}:" in
    *:"$XDG_DATA_HOME/dfx/bin":*)
        ;;
    *)
        # Prepending path in case a dfxvm/dfx on a system path needs to be overridden
        export PATH="$XDG_DATA_HOME/dfx/bin:$PATH"
        ;;
esac
