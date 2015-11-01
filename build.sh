
out="assets/out_$(date '+%Y-%m-%d_%H:%M:%S').png"

cargo run --release assets/image.rgb assets/out.rgb && convert -depth 8 -size 800x1200 assets/out.rgb "$out" && qiv "$out"
