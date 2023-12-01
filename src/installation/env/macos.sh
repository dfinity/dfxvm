#!/bin/sh
# dfxvm shell setup
# affix colons on either side of $PATH to simplify matching
case ":${PATH}:" in
    *:"$HOME/Library/Application Support/org.dfinity.dfx/bin":*)
        ;;
    *)
        # Prepending path in case a dfxvm/dfx on a system path needs to be overridden
        export PATH="$HOME/Library/Application Support/org.dfinity.dfx/bin:$PATH"
        ;;
esac
