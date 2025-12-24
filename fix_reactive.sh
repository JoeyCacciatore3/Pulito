#!/bin/bash
# Fix all reactive statements in chart files for Svelte 5 runes mode

for file in src/lib/components/charts/*.svelte; do
    echo "Fixing $file..."
    
    # Convert $: var = expr; to let var = $derived(expr);
    sed -i 's/^\([[:space:]]*\)\$:\([[:space:]]*\)\([a-zA-Z_][a-zA-Z0-9_]*\)[[:space:]]*=[[:space:]]*\(.*\);$/\1let \3 = $derived(\4);/g' "$file"
    
    # Convert $: var = { to let var = $derived({
    sed -i 's/^\([[:space:]]*\)\$:\([[:space:]]*\)\([a-zA-Z_][a-zA-Z0-9_]*\)[[:space:]]*=[[:space:]]*{$/\1let \3 = $derived({/g' "$file"
    
    # Convert $: var = ((() => { to let var = $derived(((() => {
    sed -i 's/^\([[:space:]]*\)\$:\([[:space:]]*\)\([a-zA-Z_][a-zA-Z0-9_]*\)[[:space:]]*=[[:space:]]*\(\(\(\) => {\)$/\1let \3 = $derived(\4/g' "$file"
    
    # Close }); with })); for $derived
    sed -i 's/^\([[:space:]]*\)\}\);$/\1}));/g' "$file"
    
done
