<script>
  import { onMount } from "svelte";
  let time = "";
  let period = "";
  let minutesLeft = "";

  const updateTime = () => {
    const now = new Date();
    time = now.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    const hour = now.getHours();
    const minute = now.getMinutes();
    const totalMinutes = hour * 60 + minute;
    let endMinutes = null;

    if (totalMinutes >= 915 || hour < 3) {
      period = "放課後";
      minutesLeft = "";
      return;
    }

    if (totalMinutes >= 535 && totalMinutes < 585) { // 8:55 - 9:45
      period = "1限";
      endMinutes = 585
    } else if (totalMinutes >= 585 && totalMinutes < 595) { // 9:45 - 9:55
      period = "休憩";
      endMinutes = 595; // 9:55
    } else if (totalMinutes >= 595 && totalMinutes < 645) { // 9:55 - 10:45
      period = "2限";
      endMinutes = 645;
    } else if (totalMinutes >= 645 && totalMinutes < 655) { // 10:45 - 10:55
      period = "休憩";
      endMinutes = 655;
    } else if (totalMinutes >= 655 && totalMinutes < 705) { // 10:55 - 11:45
      period = "3限";
      endMinutes = 705;
    } else if (totalMinutes >= 705 && totalMinutes < 715) { // 11:45 - 11:55
      period = "休憩";
      endMinutes = 715;
    } else if (totalMinutes >= 715 && totalMinutes < 765) { // 11:55 - 12:45
      period = "4限";
      endMinutes = 765;
    } else if (totalMinutes >= 765 && totalMinutes < 805) { // 12:45 - 13:25
      period = "昼休み";
      endMinutes = 805;
    } else if (totalMinutes >= 805 && totalMinutes < 855) { // 13:25 - 14:15
      period = "5限";
      endMinutes = 855;
    } else if (totalMinutes >= 855 && totalMinutes < 865) { // 14:15 - 14:25
      period = "休憩";
      endMinutes = 865;
    } else if (totalMinutes >= 865 && totalMinutes < 915) { // 14:25 - 15:15
      period = "6限";
      endMinutes = 915;
    }
    if (endMinutes !== null) {
      minutesLeft = endMinutes - totalMinutes;
      if (minutesLeft < 0) {
        minutesLeft = 0; // マイナスにならないように
      }
    } else {
      minutesLeft = "";
    }
  };



  onMount(() => {
    updateTime();
    setInterval(updateTime, 1000);
  });
</script>

<main
  style="
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    font-size: 1em;
    text-align: center;
    user-select: none;
    margin: 0;
    padding: 0;
    transform: translateY(-3px);
  "
>
  <div>
    {time}
    {#if period}
      / {period}
      {#if period !== "放課後"}
        / あと{minutesLeft}分
      {/if}
    {/if
    }
  </div>
  <button aria-label="設定"
    style="
    position: absolute;
    top: -0.40em;
    right: 0.1em;
    font-size: 1em;
    padding: 0.2em 0.4em;
    cursor: pointer;
    border: none;
    background: transparent;
    outline: none;
    width: 40px;
    height: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
    "
  >
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="40"
      height="40"
      viewBox="0 0 40 40"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path d="M10.325 4.317c.426-1.756 3.084-1.756 3.51 0a1.724 1.724 0 0 0 2.591 1.1c1.518-.888 3.286.88 2.4 2.397a1.724 1.724 0 0 0 1.098 2.592c1.756.426 1.756 3.084 0 3.51a1.724 1.724 0 0 0-1.098 2.591c.887 1.518-.882 3.286-2.4 2.4a1.724 1.724 0 0 0-2.591 1.098c-.426 1.756-3.084 1.756-3.51 0a1.724 1.724 0 0 0-2.592-1.098c-1.518.886-3.286-.882-2.4-2.4a1.724 1.724 0 0 0-1.098-2.591c-1.756-.426-1.756-3.084 0-3.51a1.724 1.724 0 0 0 1.098-2.592c-.886-1.518.882-3.285 2.4-2.397a1.724 1.724 0 0 0 2.592-1.1z" />
      <circle cx="12" cy="12" r="3" />
    </svg>
  </button>
</main>
