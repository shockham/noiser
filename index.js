import('./pkg')
  .then(rust_module => {
    let fm = null;
    let seq_interval = null;
    let current_step = 0;

    const play_button = document.getElementById("play");
    play_button.addEventListener("click", event => {
      if (fm === null) {
        fm = new rust_module.FmOsc();
        fm.set_note(50);
        fm.set_fm_frequency(parseFloat(fm_freq.value));
        fm.set_fm_amount(parseFloat(fm_amount.value));
        fm.set_filter(parseFloat(filter.value));
        fm.set_filter_q(parseFloat(filter_q.value));
        fm.set_gain(0.8);
        startLoop();
      } else {
        fm.free();
        fm = null;
        clearInterval(seq_interval);
        current_step = 0;
      }
    });

    const fm_freq = document.getElementById("fm_freq");
    fm_freq.addEventListener("input", event => {
      if (fm) {
        fm.set_fm_frequency(parseFloat(event.target.value));
      }
    });

    const fm_amount = document.getElementById("fm_amount");
    fm_amount.addEventListener("input", event => {
      if (fm) {
        fm.set_fm_amount(parseFloat(event.target.value));
      }
    });

    const filter = document.getElementById("filter");
    filter.addEventListener("input", event => {
      if (fm) {
        fm.set_filter(parseFloat(event.target.value));
      }
    });

    const filter_q = document.getElementById("filter_q");
    filter_q.addEventListener("input", event => {
      if (fm) {
        fm.set_filter_q(parseFloat(event.target.value));
      }
    });

    let step_inputs = [...Array(16).keys()]
          .map(x => {
              return {
                'step_input': document.getElementById(`step_${x + 1}`),
                'freq_input': document.getElementById(`freq_${x + 1}`)
              };
          });

    function startLoop() {
        seq_interval = setInterval(() => {
          const { step_input, freq_input } = step_inputs[current_step];

          step_inputs.map(({step_input, freq_input}) => {
              step_input.parentElement.style.backgroundColor = 'transparent'
              freq_input.parentElement.style.color = "#333";
          });
          freq_input.parentElement.style.backgroundColor = "#333";
          freq_input.parentElement.style.color = "#fff";

          if (fm) {
            if (step_input.checked) {
              fm.set_gain(0.8);
              fm.set_note(parseFloat(freq_input.value));
            } else {
              fm.set_gain(0.0);
            }
          }

          current_step++;
          if (current_step > 15) {
            current_step = 0;
          }
        }, 250);
    }
  })
  .catch(console.error);
