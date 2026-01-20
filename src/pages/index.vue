<template>
  <div>
    <div class="landing-hero">
      <h1 class="main-header">
        <IntlFormatted :message-id="messages.thePlaceForMinecraft">
          <template #~content>
            <strong class="main-header-strong">
              {{ formatMessage(contentTypeMessages.plugins) }}
            </strong>
          </template>
        </IntlFormatted>
      </h1>
      <h2>
        {{ formatMessage(messages.discoverHeading) }}
      </h2>
      <div class="button-group">
        <ButtonStyled color="brand" size="large">
          <nuxt-link to="/discover/plugins">
            <CompassIcon aria-hidden="true" />
            {{ formatMessage(messages.discoverPlugins) }}
          </nuxt-link>
        </ButtonStyled>
      </div>
    </div>
    <div class="users-section-outer">
      <div v-if="rows" class="projects-showcase">
        <div v-for="(row, index) in rows" :key="index" class="row">
          <div v-for="n in 2" :key="n" class="row__content" :class="{ offset: index % 2 }">
            <nuxt-link v-for="plugin in row" :key="plugin.id" class="project button-animation"
              :to="`/plugin/${plugin.id}`">
              <Avatar :src="plugin.icon_url" :alt="plugin.name" size="sm" loading="lazy" />
              <div class="project-info">
                <span class="title">
                  {{ plugin.name }}
                </span>
                <span class="description">
                  {{ plugin.summary }}
                </span>
              </div>
            </nuxt-link>
          </div>
        </div>
      </div>
      <div v-else class="relative z-[10] w-full text-center text-xl font-bold text-contrast">
        {{ formatMessage(messages.failedToLoadRandomProjects) }}
      </div>
      <div class="projects-transition" />
      <div class="users-section">
        <div class="feature-blob">
          <div class="blob-text">
            <h3>{{ formatMessage(messages.findWhatYouWantHeading) }}</h3>
            <p>
              {{ formatMessage(messages.findWhatYouWantDescription) }}
            </p>
          </div>
          <div class="blob-demonstration gradient-border bigger">
            <div class="demo-search">
              <div class="search-controls">
                <div class="iconified-input">
                  <label class="hidden" for="search">{{
                    formatMessage(messages.searchLabel)
                    }}</label>
                  <SearchIcon aria-hidden="true" />
                  <input id="search" v-model="searchQuery" type="search" name="search"
                    :placeholder="formatMessage(messages.searchPlaceholder)" autocomplete="off"
                    @input="updateSearchProjects" />
                </div>
                <div class="sort-by">
                  <span class="label">{{
                    formatMessage(messages.sortByLabel)
                    }}</span>
                  <DropdownSelect v-model="sortType" name="sort-select" class="selector"
                    :options="['downloads', 'stars', 'updated', 'newest']" :display-name="(value: unknown) =>
                        String(value).charAt(0).toUpperCase() +
                        String(value).slice(1)
                      " @change="updateSearchProjects" />
                </div>
              </div>
              <div class="results display-mode--list">
                <ProjectCard v-for="result in searchProjects" :id="result.id" :key="result.id"
                  class="small-mode gradient-border" type="plugin" :author="result.author" :name="result.name"
                  :description="result.summary" :created-at="result.created_at" :updated-at="result.updated_at"
                  :downloads="result.downloads.toString()" :stars="result.stars.toString()" :icon-url="result.icon_url"
                  :categories="result.categories.slice(0, 3)" :search="true" :show-updated-date="true" />
              </div>
            </div>
          </div>
        </div>
        <div class="feature-blob">
          <div class="blob-text">
            <h3>{{ formatMessage(messages.playWithLauncherHeading) }}</h3>
            <p>
              <IntlFormatted :message-id="messages.playWithLauncherDescription">
                <template #link="{ children }">
                  <nuxt-link class="title-link" to="/app">
                    <component :is="() => children" />
                  </nuxt-link>
                </template>
              </IntlFormatted>
            </p>
          </div>
          <img v-if="$theme.active === 'light'" src="~/assets/images/app/AllayLauncher_light.webp"
            :alt="formatMessage(messages.launcherGraphicAlt)" class="launcher-screen" />
          <img v-else src="~/assets/images/app/AllayLauncher_dark.webp"
            :alt="formatMessage(messages.launcherGraphicAlt)" class="launcher-screen" />
        </div>
      </div>
    </div>
    <div class="creator-section">
      <div class="section-header">
        <div class="section-label blue">
          {{ formatMessage(messages.forCreatorsLabel) }}
        </div>
        <h2 class="section-tagline">
          {{ formatMessage(messages.shareContentTagline) }}
        </h2>
        <p class="section-description">
          {{ formatMessage(messages.creatorsDescription) }}
        </p>
      </div>
      <div class="features">
        <!-- Discovery -->
        <div class="feature gradient-border">
          <div class="icon gradient-border">
            <svg viewBox="0 0 37 37" fill="none">
              <path
                d="M18.4764 0C18.3765 0 18.2765 0.0111037 18.1766 0.0444178C8.23734 0.210982 0.197119 8.2617 0.0416464 18.2002C-0.0138821 18.4001 -0.0138821 18.6221 0.0416464 18.822C0.208223 28.7605 8.25955 36.8001 18.1988 36.9667C18.3987 37.0111 18.6208 37.0111 18.8207 36.9667C28.7599 36.789 36.8001 28.7383 36.9667 18.7998C37.0111 18.5999 37.0111 18.3778 36.9667 18.178C36.789 8.2506 28.7599 0.210982 18.8318 0.0444178C18.7096 0.0111037 18.5986 0 18.4764 0ZM17.0771 2.93157V5.70768C17.066 6.21849 17.3437 6.69598 17.7768 6.95138C18.221 7.21789 18.7762 7.21789 19.2205 6.95138C19.6536 6.69598 19.9312 6.21849 19.9201 5.70768V2.93157C27.4273 3.60894 33.3908 9.57203 34.0571 17.0786H31.2919C30.781 17.0675 30.3035 17.3451 30.0481 17.7782C29.7816 18.2224 29.7816 18.7776 30.0481 19.2218C30.3035 19.6549 30.781 19.9325 31.2919 19.9214H34.0571C33.3908 27.428 27.4273 33.3911 19.9201 34.0573V31.2923C19.9312 30.9037 19.7757 30.5372 19.5092 30.2596C19.2316 29.9931 18.8651 29.8376 18.4764 29.8487C17.6879 29.8598 17.066 30.5039 17.0771 31.2923V34.0573C9.56997 33.3911 3.60644 27.428 2.92902 19.9214H5.70534C6.21618 19.9325 6.6937 19.6549 6.94913 19.2218C7.21565 18.7776 7.21565 18.2224 6.94913 17.7782C6.6937 17.3451 6.21618 17.0675 5.70534 17.0786H2.92902C3.60644 9.57203 9.56997 3.60894 17.0771 2.93157ZM25.606 11.3932L15.6557 15.6573L11.3912 25.6068L21.3416 21.3427L25.606 11.3932Z"
                fill="url(#paint0_linear_255_241)" />
              <defs>
                <linearGradient id="paint0_linear_255_241" x1="18.5" y1="0" x2="18.5" y2="37"
                  gradientUnits="userSpaceOnUse">
                  <stop stop-color="#C1E1B1" />
                  <stop offset="1" stop-color="#A7BDE6" />
                </linearGradient>
              </defs>
            </svg>
          </div>
          <h3>{{ formatMessage(creatorFeatureMessages.discoveryTitle) }}</h3>
          <p>
            {{ formatMessage(creatorFeatureMessages.discoveryDescription) }}
          </p>
        </div>
        <!-- Auto Tracking -->
        <div class="feature gradient-border">
          <div class="icon gradient-border">
            <svg viewBox="0 0 42 32" fill="none">
              <path
                d="M21 0C12.3149 0 5.25 7.06494 5.25 15.75H0L6.5625 22.6406L13.125 15.75H7.875C7.875 8.47998 13.73 2.625 21 2.625C23.9121 2.625 26.5884 3.56836 28.752 5.16797L30.3105 3.04541C27.6958 1.12793 24.4761 0 21 0ZM21 7.875C18.1187 7.875 15.75 10.2437 15.75 13.125C15.75 14.5298 16.3242 15.8115 17.2266 16.7549C14.7964 18.0981 13.125 20.6616 13.125 23.625H15.75C15.75 20.7129 18.0879 18.375 21 18.375C23.9121 18.375 26.25 20.7129 26.25 23.625H28.875C28.875 20.6616 27.2036 18.0981 24.7734 16.7549C25.6758 15.8115 26.25 14.5298 26.25 13.125C26.25 10.2437 23.8813 7.875 21 7.875ZM35.4375 9.1875L28.875 15.75H34.125C34.125 23.02 28.27 28.875 21 28.875C18.0879 28.875 15.4116 27.9316 13.248 26.332L11.6895 28.4546C14.3042 30.3721 17.5239 31.5 21 31.5C29.6851 31.5 36.75 24.4351 36.75 15.75H42L35.4375 9.1875ZM21 10.5C22.4663 10.5 23.625 11.6587 23.625 13.125C23.625 14.5913 22.4663 15.75 21 15.75C19.5337 15.75 18.375 14.5913 18.375 13.125C18.375 11.6587 19.5337 10.5 21 10.5Z"
                fill="url(#paint0_linear_255_234)" />
              <defs>
                <linearGradient id="paint0_linear_255_234" x1="21" y1="0" x2="21" y2="31.5"
                  gradientUnits="userSpaceOnUse">
                  <stop stop-color="#C1E1B1" />
                  <stop offset="1" stop-color="#A7BDE6" />
                </linearGradient>
              </defs>
            </svg>
          </div>
          <h3>{{ formatMessage(creatorFeatureMessages.autoTrackingTitle) }}</h3>
          <p>
            {{ formatMessage(creatorFeatureMessages.autoTrackingDescription) }}
          </p>
        </div>
        <!-- Statistics -->
        <div class="feature gradient-border">
          <div class="icon gradient-border">
            <svg viewBox="0 0 39 39" fill="none">
              <path
                d="M0.875 0.875V38.25H38.25V35.375H3.75V0.875H0.875ZM18.125 0.875C17.3625 0.875 16.6312 1.1779 16.0921 1.71707C15.5529 2.25623 15.25 2.9875 15.25 3.75C15.25 4.5125 15.5529 5.24376 16.0921 5.78293C16.6312 6.3221 17.3625 6.625 18.125 6.625C18.8875 6.625 19.6188 6.3221 20.1579 5.78293C20.6971 5.24376 21 4.5125 21 3.75C21 2.9875 20.6971 2.25623 20.1579 1.71707C19.6188 1.1779 18.8875 0.875 18.125 0.875ZM8.0625 3.75C7.68125 3.75 7.31562 3.90145 7.04603 4.17103C6.77645 4.44062 6.625 4.80625 6.625 5.1875C6.625 5.56875 6.77645 5.93438 7.04603 6.20397C7.31562 6.47355 7.68125 6.625 8.0625 6.625C8.44375 6.625 8.80938 6.47355 9.07897 6.20397C9.34855 5.93438 9.5 5.56875 9.5 5.1875C9.5 4.80625 9.34855 4.44062 9.07897 4.17103C8.80938 3.90145 8.44375 3.75 8.0625 3.75ZM28.1875 3.75C27.8063 3.75 27.4406 3.90145 27.171 4.17103C26.9014 4.44062 26.75 4.80625 26.75 5.1875C26.75 5.56875 26.9014 5.93438 27.171 6.20397C27.4406 6.47355 27.8063 6.625 28.1875 6.625C28.5687 6.625 28.9344 6.47355 29.204 6.20397C29.4736 5.93438 29.625 5.56875 29.625 5.1875C29.625 4.80625 29.4736 4.44062 29.204 4.17103C28.9344 3.90145 28.5687 3.75 28.1875 3.75ZM36.8125 3.75C36.4313 3.75 36.0656 3.90145 35.796 4.17103C35.5265 4.44062 35.375 4.80625 35.375 5.1875C35.375 5.56875 35.5265 5.93438 35.796 6.20397C36.0656 6.47355 36.4313 6.625 36.8125 6.625C37.1937 6.625 37.5594 6.47355 37.829 6.20397C38.0985 5.93438 38.25 5.56875 38.25 5.1875C38.25 4.80625 38.0985 4.44062 37.829 4.17103C37.5594 3.90145 37.1937 3.75 36.8125 3.75ZM26.75 12.375C25.9875 12.375 25.2562 12.6779 24.7171 13.2171C24.1779 13.7562 23.875 14.4875 23.875 15.25C23.875 16.0125 24.1779 16.7438 24.7171 17.2829C25.2562 17.8221 25.9875 18.125 26.75 18.125C27.5125 18.125 28.2438 17.8221 28.7829 17.2829C29.3221 16.7438 29.625 16.0125 29.625 15.25C29.625 14.4875 29.3221 13.7562 28.7829 13.2171C28.2438 12.6779 27.5125 12.375 26.75 12.375ZM8.0625 15.25C7.68125 15.25 7.31562 15.4015 7.04603 15.671C6.77645 15.9406 6.625 16.3063 6.625 16.6875C6.625 17.0687 6.77645 17.4344 7.04603 17.704C7.31562 17.9735 7.68125 18.125 8.0625 18.125C8.44375 18.125 8.80938 17.9735 9.07897 17.704C9.34855 17.4344 9.5 17.0687 9.5 16.6875C9.5 16.3063 9.34855 15.9406 9.07897 15.671C8.80938 15.4015 8.44375 15.25 8.0625 15.25ZM36.8125 15.25C36.4313 15.25 36.0656 15.4015 35.796 15.671C35.5265 15.9406 35.375 16.3063 35.375 16.6875C35.375 17.0687 35.5265 17.4344 35.796 17.704C36.0656 17.9735 36.4313 18.125 36.8125 18.125C37.1937 18.125 37.5594 17.9735 37.829 17.704C38.0985 17.4344 38.25 17.0687 38.25 16.6875C38.25 16.3063 38.0985 15.9406 37.829 15.671C37.5594 15.4015 37.1937 15.25 36.8125 15.25ZM9.5 23.875C8.7375 23.875 8.00624 24.1779 7.46707 24.7171C6.9279 25.2562 6.625 25.9875 6.625 26.75C6.625 27.5125 6.9279 28.2438 7.46707 28.7829C8.00624 29.3221 8.7375 29.625 9.5 29.625C10.2625 29.625 10.9938 29.3221 11.5329 28.7829C12.0721 28.2438 12.375 27.5125 12.375 26.75C12.375 25.9875 12.0721 25.2562 11.5329 24.7171C10.9938 24.1779 10.2625 23.875 9.5 23.875ZM19.5625 26.75C19.1813 26.75 18.8156 26.9014 18.546 27.171C18.2765 27.4406 18.125 27.8063 18.125 28.1875C18.125 28.5687 18.2765 28.9344 18.546 29.204C18.8156 29.4736 19.1813 29.625 19.5625 29.625C19.9437 29.625 20.3094 29.4736 20.579 29.204C20.8485 28.9344 21 28.5687 21 28.1875C21 27.8063 20.8485 27.4406 20.579 27.171C20.3094 26.9014 19.9437 26.75 19.5625 26.75ZM28.1875 26.75C27.8063 26.75 27.4406 26.9014 27.171 27.171C26.9014 27.4406 26.75 27.8063 26.75 28.1875C26.75 28.5687 26.9014 28.9344 27.171 29.204C27.4406 29.4736 27.8063 29.625 28.1875 29.625C28.5687 29.625 28.9344 29.4736 29.204 29.204C29.4736 28.9344 29.625 28.5687 29.625 28.1875C29.625 27.8063 29.4736 27.4406 29.204 27.171C28.9344 26.9014 28.5687 26.75 28.1875 26.75ZM36.8125 26.75C36.4313 26.75 36.0656 26.9014 35.796 27.171C35.5265 27.4406 35.375 27.8063 35.375 28.1875C35.375 28.5687 35.5265 28.9344 35.796 29.204C36.0656 29.4736 36.4313 29.625 36.8125 29.625C37.1937 29.625 37.5594 29.4736 37.829 29.204C38.0985 28.9344 38.25 28.5687 38.25 28.1875C38.25 27.8063 38.0985 27.4406 37.829 27.171C37.5594 26.9014 37.1937 26.75 36.8125 26.75Z"
                fill="url(#paint0_linear_127_323)" />
              <defs>
                <linearGradient id="paint0_linear_127_323" x1="19.5625" y1="0.875" x2="19.5625" y2="38.25"
                  gradientUnits="userSpaceOnUse">
                  <stop stop-color="#C1E1B1" />
                  <stop offset="1" stop-color="#A7BDE6" />
                </linearGradient>
              </defs>
            </svg>
          </div>
          <h3>{{ formatMessage(creatorFeatureMessages.statisticsTitle) }}</h3>
          <p>
            {{ formatMessage(creatorFeatureMessages.statisticsDescription) }}
          </p>
        </div>
        <!-- Open Source -->
        <div class="feature gradient-border">
          <div class="icon gradient-border">
            <svg viewBox="0 0 42 39" fill="none">
              <path
                d="M5.1875 0.875C2.81787 0.875 0.875 2.81787 0.875 5.1875C0.875 7.55713 2.81787 9.5 5.1875 9.5C5.8501 9.5 6.46777 9.33154 7.0293 9.06201L13.5767 15.6094C12.8242 16.7437 12.375 18.1025 12.375 19.5625C12.375 20.708 12.667 21.7861 13.1611 22.752L7.42236 27.3901C6.771 26.9971 6.00732 26.75 5.1875 26.75C2.81787 26.75 0.875 28.6929 0.875 31.0625C0.875 33.4321 2.81787 35.375 5.1875 35.375C7.55713 35.375 9.5 33.4321 9.5 31.0625C9.5 30.5571 9.39893 30.0854 9.23047 29.6362L14.9355 25.0093C16.1934 26.0762 17.7993 26.75 19.5625 26.75C20.3374 26.75 21.0674 26.5928 21.7637 26.3682L24.8296 31.2759C24.2456 32.0171 23.875 32.9268 23.875 33.9375C23.875 36.3071 25.8179 38.25 28.1875 38.25C30.5571 38.25 32.5 36.3071 32.5 33.9375C32.5 31.5679 30.5571 29.625 28.1875 29.625C27.873 29.625 27.5586 29.6699 27.2554 29.7261L24.2793 24.9419C25.2339 24.0996 25.9639 23.0103 26.3682 21.7637L32.5562 23.0327C32.8594 25.1216 34.6562 26.75 36.8125 26.75C39.1821 26.75 41.125 24.8071 41.125 22.4375C41.125 20.0679 39.1821 18.125 36.8125 18.125C35.2627 18.125 33.9038 18.9785 33.1401 20.2139L26.6826 18.8887C26.5029 17.0132 25.6157 15.3511 24.2568 14.1719L27.2441 9.3877C27.5474 9.45508 27.8618 9.5 28.1875 9.5C30.5571 9.5 32.5 7.55713 32.5 5.1875C32.5 2.81787 30.5571 0.875 28.1875 0.875C25.8179 0.875 23.875 2.81787 23.875 5.1875C23.875 6.18701 24.2344 7.10791 24.8184 7.83789L21.7524 12.7568C21.0562 12.5322 20.3262 12.375 19.5625 12.375C18.1025 12.375 16.7437 12.8242 15.6094 13.5767L9.06201 7.0293C9.33154 6.46777 9.5 5.8501 9.5 5.1875C9.5 2.81787 7.55713 0.875 5.1875 0.875ZM5.1875 3.75C5.99609 3.75 6.625 4.37891 6.625 5.1875C6.625 5.99609 5.99609 6.625 5.1875 6.625C4.37891 6.625 3.75 5.99609 3.75 5.1875C3.75 4.37891 4.37891 3.75 5.1875 3.75ZM28.1875 3.75C28.9961 3.75 29.625 4.37891 29.625 5.1875C29.625 5.99609 28.9961 6.625 28.1875 6.625C27.3789 6.625 26.75 5.99609 26.75 5.1875C26.75 4.37891 27.3789 3.75 28.1875 3.75ZM19.5625 15.25C21.9658 15.25 23.875 17.1592 23.875 19.5625C23.875 21.1123 23.0664 22.4487 21.8535 23.2124C21.7075 23.2686 21.5728 23.3359 21.4492 23.437C20.8765 23.7065 20.2363 23.875 19.5625 23.875C17.1592 23.875 15.25 21.9658 15.25 19.5625C15.25 17.1592 17.1592 15.25 19.5625 15.25ZM36.8125 21C37.6211 21 38.25 21.6289 38.25 22.4375C38.25 23.2461 37.6211 23.875 36.8125 23.875C36.0039 23.875 35.375 23.2461 35.375 22.4375C35.375 21.6289 36.0039 21 36.8125 21ZM5.1875 29.625C5.99609 29.625 6.625 30.2539 6.625 31.0625C6.625 31.8711 5.99609 32.5 5.1875 32.5C4.37891 32.5 3.75 31.8711 3.75 31.0625C3.75 30.2539 4.37891 29.625 5.1875 29.625ZM28.1875 32.5C28.9961 32.5 29.625 33.1289 29.625 33.9375C29.625 34.7461 28.9961 35.375 28.1875 35.375C27.3789 35.375 26.75 34.7461 26.75 33.9375C26.75 33.1289 27.3789 32.5 28.1875 32.5Z"
                fill="url(#paint0_linear_127_281)" />
              <defs>
                <linearGradient id="paint0_linear_127_281" x1="21" y1="0.875" x2="21" y2="38.25"
                  gradientUnits="userSpaceOnUse">
                  <stop stop-color="#C1E1B1" />
                  <stop offset="1" stop-color="#A7BDE6" />
                </linearGradient>
              </defs>
            </svg>
          </div>
          <h3>{{ formatMessage(creatorFeatureMessages.openSourceTitle) }}</h3>
          <p>
            {{ formatMessage(creatorFeatureMessages.openSourceDescription) }}
          </p>
        </div>
      </div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { CompassIcon, SearchIcon } from '@modrinth/assets'
import {
  Avatar,
  ButtonStyled,
  defineMessages,
  DropdownSelect,
  IntlFormatted,
  useVIntl,
} from '@modrinth/ui'
import { ref } from 'vue'

import ProjectCard from '~/components/ui/ProjectCard.vue'
import { usePluginSearch, type SortOption } from '~/composables/usePlugins'

const { formatMessage } = useVIntl()

// Use search composable
const { search, getPopular } = usePluginSearch()

const searchQuery = ref('')
const sortType = ref<SortOption>('downloads')

// Plugins list for showcase
const pluginsList = ref<AllayIndex.PluginSummary[]>([])

// Compute rows for the showcase from plugins list
const rows = computed(() => {
  const plugins = pluginsList.value?.slice(0, 40) ?? []
  if (plugins.length === 0) return undefined
  const val = Math.ceil(plugins.length / 3)
  return [
    plugins.slice(0, val),
    plugins.slice(val, val * 2),
    plugins.slice(val * 2, val * 3),
  ]
})

// Search results
const searchProjects = ref<AllayIndex.PluginSummary[]>([])

// Initialize with popular plugins
onMounted(async () => {
  // Load plugins for showcase
  pluginsList.value = await getPopular(40)
  // Load initial search results
  searchProjects.value = await getPopular(5)
})

async function updateSearchProjects(): Promise<void> {
  searchProjects.value = await search(
    { query: searchQuery.value },
    { sort: sortType.value, limit: 5 },
  )
}

const messages = defineMessages({
  thePlaceForMinecraft: {
    id: 'landing.heading.the-place-for-minecraft',
    defaultMessage: 'The place for Minecraft {content}',
  },
  discoverHeading: {
    id: 'landing.subheading',
    defaultMessage:
      'Discover, play, and share Minecraft content through our open-source platform built for the community.',
  },
  discoverPlugins: {
    id: 'landing.button.discover-plugins',
    defaultMessage: 'Discover plugins',
  },
  goToDashboard: {
    id: 'landing.button.go-to-dashboard',
    defaultMessage: 'Go to dashboard',
  },
  failedToLoadRandomProjects: {
    id: 'landing.error.failedToLoadRandomProjects',
    defaultMessage: 'Failed to load random projects :(',
  },
  forPlayersLabel: {
    id: 'landing.section.for-players.label',
    defaultMessage: 'For Players',
  },
  forCreatorsLabel: {
    id: 'landing.section.for-creators.label',
    defaultMessage: 'For Creators',
  },
  discoverCreationsTagline: {
    id: 'landing.section.for-players.tagline',
    defaultMessage: 'Discover over {count} creations',
  },
  shareContentTagline: {
    id: 'landing.section.for-creators.tagline',
    defaultMessage: 'Share your content with the world',
  },
  playersDescription: {
    id: 'landing.section.for-players.description',
    defaultMessage:
      'From magical biomes to cursed dungeons, you can be sure to find content to bring your gameplay to the next level.',
  },
  creatorsDescription: {
    id: 'landing.section.for-creators.description',
    defaultMessage:
      'Give an online home to your creations and reach a massive audience of dedicated players.',
  },
  findWhatYouWantHeading: {
    id: 'landing.feature.search.heading',
    defaultMessage: 'Find what you want, quickly and easily',
  },
  findWhatYouWantDescription: {
    id: 'landing.feature.search.description',
    defaultMessage:
      "AllayHub's lightning-fast search and powerful filters let you find what you want as you type.",
  },
  followProjectsHeading: {
    id: 'landing.feature.follow.heading',
    defaultMessage: 'Follow projects you love',
  },
  followProjectsDescription: {
    id: 'landing.feature.follow.description',
    defaultMessage:
      'Get notified every time your favorite projects update and stay in the loop.',
  },
  playWithLauncherHeading: {
    id: 'landing.feature.launcher.heading',
    defaultMessage: 'Play with your favorite launcher',
  },
  playWithLauncherDescription: {
    id: 'landing.feature.launcher.description',
    defaultMessage:
      "AllayHub's open-source API lets launchers add deep integration with AllayHub. You can use AllayHub through <link>our own app</link> and some of the most popular launchers like ATLauncher, MultiMC, and Prism Launcher.",
  },
  searchPlaceholder: {
    id: 'landing.search.placeholder',
    defaultMessage: 'Search...',
  },
  searchLabel: {
    id: 'landing.search.label',
    defaultMessage: 'Search',
  },
  sortByLabel: {
    id: 'landing.search.sort-by.label',
    defaultMessage: 'Sort by',
  },
  notificationsHeading: {
    id: 'landing.notifications.heading',
    defaultMessage: 'Notifications',
  },
  hasBeenUpdated: {
    id: 'landing.notifications.has-been-updated',
    defaultMessage: '{title} has been updated!',
  },
  versionReleased: {
    id: 'landing.notifications.version-released',
    defaultMessage: 'Version {version} has been released for {gameVersion}',
  },
  receivedTime: {
    id: 'landing.notifications.received-time',
    defaultMessage: 'Received {time}',
  },
  launcherGraphicAlt: {
    id: 'landing.launcher.graphic-alt',
    defaultMessage: 'A simplified representation of an Allay server window.',
  },
  allayLauncherLabel: {
    id: 'landing.launcher.allay-launcher-label',
    defaultMessage: 'AllayLauncher',
  },
})

const contentTypeMessages = defineMessages({
  plugins: {
    id: 'landing.heading.the-place-for-minecraft.plugins',
    defaultMessage: 'plugins',
  },
})

const creatorFeatureMessages = defineMessages({
  discoveryTitle: {
    id: 'landing.creator.feature.discovery.title',
    defaultMessage: 'Discovery',
  },
  discoveryDescription: {
    id: 'landing.creator.feature.discovery.description',
    defaultMessage:
      'Get your plugin discovered by AllayMC server owners through search, categories, and our home page.',
  },
  autoTrackingTitle: {
    id: 'landing.creator.feature.auto-tracking.title',
    defaultMessage: 'Auto Tracking',
  },
  autoTrackingDescription: {
    id: 'landing.creator.feature.auto-tracking.description',
    defaultMessage:
      'Your GitHub repository is automatically indexed. README, releases, changelogs, and version history are synced automatically.',
  },
  statisticsTitle: {
    id: 'landing.creator.feature.statistics.title',
    defaultMessage: 'Statistics',
  },
  statisticsDescription: {
    id: 'landing.creator.feature.statistics.description',
    defaultMessage:
      'Track your plugin popularity with GitHub stars, forks, and download counts from releases.',
  },
  openSourceTitle: {
    id: 'landing.creator.feature.open-source.title',
    defaultMessage: 'Open Source',
  },
  openSourceDescription: {
    id: 'landing.creator.feature.open-source.description',
    defaultMessage:
      'AllayHub is fully open source. Submit plugins via PR, contribute to the platform, and help grow the AllayMC ecosystem.',
  },
})
</script>

<style lang="scss" scoped>
.landing-hero {
  background-image: var(--landing-maze-bg);
  background-size: cover;
  object-fit: contain;
  padding: 6rem 1rem 12rem 1rem;

  display: flex;
  justify-content: center;
  align-items: center;
  text-align: center;
  flex-direction: column;

  .modrinth-icon {
    width: 13rem;
    height: 13rem;
    margin-bottom: 2.5rem;
  }

  h2 {
    font-size: 1.25rem;
    line-height: 125%;
    margin: 0 0 1.625rem;
    font-weight: 400;
    line-break: loose;
    color: var(--landing-color-subheading);
    max-width: 50rem;
  }

  .button-group {
    width: fit-content;
    gap: 1.25rem;
    margin: 0 auto 5rem;
    justify-content: center;
  }
}

.users-section-outer {
  position: relative;
  background: var(--landing-maze-outer-bg);
  width: 100%;

  &:before {
    content: '';
    position: absolute;
    z-index: 1;
    inset: 0;
    background: linear-gradient(180deg,
        var(--landing-transition-gradient-end) 0%,
        var(--landing-transition-gradient-start) 100%);
    height: 12.5rem;
    width: 100%;
  }

  .projects-transition {
    position: absolute;
    top: calc(-12.5rem);
    width: 100%;
    height: 12.5rem;
    background: linear-gradient(0deg,
        var(--landing-transition-gradient-end) 0%,
        var(--landing-transition-gradient-start) 100%);
  }

  .projects-showcase {
    position: absolute;
    z-index: 2;
    top: -11rem;

    .row {
      --gap: 1.5rem;

      width: 100vw;
      gap: var(--gap);
      margin-bottom: var(--gap);

      display: flex;
      overflow: hidden;
      user-select: none;

      &:hover {
        .row__content {
          animation-play-state: paused !important;
        }
      }

      .row__content {
        flex-shrink: 0;
        display: flex;
        min-width: 100%;
        gap: var(--gap);
        animation: scroll 40s linear infinite;

        @media (prefers-reduced-motion) {
          animation-play-state: paused !important;
        }

        @keyframes scroll {
          from {
            transform: translateX(0);
          }

          to {
            transform: translateX(calc(-100%));
          }
        }

        &.offset {
          animation: scroll-inverse 40s linear infinite;
          transform: translateX(-100%);

          @keyframes scroll-inverse {
            from {
              transform: translateX(calc(-100%));
            }

            to {
              transform: translateX(calc(0%));
            }
          }
        }
      }

      .project {
        position: relative;
        display: flex;

        cursor: pointer;
        padding: 1rem;
        gap: 1rem;
        border-radius: 1rem;
        border: 1px solid var(--landing-border-color);
        transition:
          background 0.5s ease-in-out,
          transform 0.05s ease-in-out;
        // Removed due to lag on mobile :(

        &:hover {
          z-index: -2;
          background: var(--landing-hover-card-gradient);
        }

        img {
          height: 3rem;
        }

        .project-info {
          box-sizing: border-box;
        }

        .title {
          color: var(--landing-color-heading);
          max-width: 13.75rem;
          overflow: hidden;
          white-space: nowrap;
          text-overflow: ellipsis;
          margin: 0;
          font-weight: 600;
          font-size: 1.25rem;
          line-height: 110%;
          display: block;
        }

        .description {
          width: 13.75rem;

          display: -webkit-box;
          -webkit-line-clamp: 2;
          line-clamp: 2;
          -webkit-box-orient: vertical;
          overflow: hidden;

          font-weight: 500;
          font-size: 0.875rem;
          line-height: 125%;
          margin: 0.25rem 0 0;
        }
      }
    }
  }

  .users-section {
    width: 100%;
    padding-top: 10rem;
    padding-bottom: 5rem;

    background: var(--landing-maze-gradient-bg);
    background-size: cover;
    background-blend-mode: multiply;

    .feature-blob {
      display: flex;
      padding: 1.25rem 1rem;
      justify-content: center;
      flex-wrap: wrap;
      column-gap: 4.375rem;
      margin: 0 0.75rem 0.75rem 0.75rem;

      &.reverse {
        flex-direction: row-reverse;
      }

      .blob-text {
        margin-top: 2.5rem;
        width: 32.5rem;
        max-width: 32.5rem;
        text-align: center;

        h3 {
          font-weight: 500;
          font-size: 2rem;
          line-height: 110%;
          color: var(--landing-color-heading);
        }

        p {
          font-weight: 400;
          font-size: 1.25rem;
          line-height: 125%;
          color: var(--landing-color-subheading);
          line-break: loose;
        }
      }

      @media screen and (min-width: 1238px) {
        padding: 5rem 1rem;

        .blob-text {
          margin-top: 5rem;
          text-align: left;
        }
      }

      .blob-demonstration {
        position: relative;
        width: 35rem;
        max-width: 35rem;
        background: var(--landing-blob-gradient);
        box-shadow: var(--landing-blob-shadow);
        // backdrop-filter: blur(6px);
        background-blend-mode: multiply;
        padding: 1rem;
        z-index: 1;

        &:after {
          content: '';
          position: absolute;
          z-index: -1;
          inset: 0 0 -0.75rem -0.75rem;

          background: linear-gradient(0deg,
              var(--color-brand) 0%,
              transparent 100%);
          opacity: 0.2;
          border-radius: 1rem;
          margin-top: auto;
          width: calc(100% + 1.5rem);
          height: 55%;
        }

        .demo-search {
          height: max-content;

          .search-controls {
            display: flex;
            justify-content: space-between;
            margin-bottom: 1rem;
            gap: 1rem;

            .iconified-input {
              width: 100%;

              svg {
                opacity: 1;
                color: var(--color-button-text-active);
              }

              input {
                box-shadow:
                  inset 0 0 0 transparent,
                  0 0 0 0.25rem var(--color-brand-shadow);
                color: var(--color-button-text-active);
              }
            }

            .sort-by {
              display: flex;
              gap: 0.75rem;
              align-items: center;

              .label {
                white-space: nowrap;
              }

              .selector {
                width: 9rem;
                white-space: nowrap;

                :deep(.animated-dropdown) {
                  width: 8rem;
                  height: 2.25rem;

                  .selected {
                    padding: var(--gap-xs) var(--gap-md);
                    font-size: 0.875rem;
                  }

                  .options .option {
                    padding: var(--gap-xs) var(--gap-md);
                    font-size: 0.875rem;
                  }
                }
              }

              @media screen and (max-width: 500px) {
                display: none;
              }
            }
          }

          .results {
            display: flex;
            flex-direction: column;
            gap: 1rem;

            .small-mode {
              background: var(--landing-card-bg);
              box-shadow: var(--landing-card-shadow);
              background-blend-mode: multiply;
              // backdrop-filter: blur(4px);
            }

            @media screen and (max-width: 450px) {
              .small-mode:nth-child(n + 2) {
                display: none;
              }
            }

            @media screen and (max-width: 500px) {
              .small-mode:nth-child(n + 3) {
                display: none;
              }
            }
          }
        }

        .notifs-demo {
          h3 {
            font-weight: 600;
            font-size: 1.5rem;
            margin: 0 0 0.75rem;
          }

          .notifications {
            display: flex;
            flex-direction: column;
            gap: 1rem;

            .notification {
              display: flex;
              gap: 1rem;
              padding: 0.75rem;
              background: var(--landing-card-bg);
              box-shadow: var(--landing-card-shadow);

              img {
                width: 4rem;
                height: 4rem;
              }

              .notif-header {
                margin: 0;
                font-weight: 600;
                font-size: 1.25rem;
              }

              .notif-desc {
                margin: 0.5rem 0;
                font-weight: 500;
                font-size: 1rem;
              }

              .date {
                display: flex;
                align-items: center;
                gap: 0.25rem;

                span {
                  font-size: 12px;
                  font-weight: 700;
                }
              }
            }
          }
        }
      }

      .launcher-screen {
        padding-top: 3rem;
        width: 40rem;
        max-width: 100%;
        height: auto;
        object-fit: contain;
        display: block;
        border-radius: 1rem;
      }
    }
  }
}

.creator-section {
  width: 100%;
  background: var(--landing-creator-gradient);
  padding: 2.5rem 0;

  .features {
    position: relative;
    display: flex;
    flex-wrap: wrap;
    max-width: 81.25rem;
    justify-content: center;
    margin: 5rem auto 0 auto;
    row-gap: 3.5rem;
    column-gap: 3rem;
    padding: 1rem;

    .feature {
      width: 34.375rem;
      padding: 1.25rem;
      z-index: 1;
      background: var(--landing-card-bg);

      .icon {
        z-index: 2;
        margin: -3.25rem 0 0.75rem 0;
        display: flex;
        align-items: center;
        justify-content: center;
        width: 4rem;
        height: 4rem;
        background: #020305;
        box-shadow:
          2px 2px 12px rgba(0, 0, 0, 0.16),
          inset 2px 2px 32px #393d5e;
        border-radius: 1rem;

        svg {
          width: 2rem;
          height: auto;
        }
      }

      .additional-label {
        width: fit-content;
        padding: 0.5rem 0.75rem;
        margin-bottom: 0.5rem;
        background: var(--landing-blue-label-bg);
        color: var(--landing-blue-label);
        border-radius: 6px;
        font-weight: 700;
        font-size: 1rem;
      }

      h3,
      p {
        font-weight: 500;
        line-height: 125%;
        margin: 0;
      }

      h3 {
        font-size: 1.25rem;
        color: var(--landing-color-heading);
        margin-bottom: 0.375rem;
      }

      p {
        font-size: 1.25rem;
        color: var(--landing-color-subheading);
      }
    }
  }
}

.logo-banner {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--landing-raw-bg);
  padding: 1rem 1rem 2rem 1rem;
  overflow: hidden;

  .modrinth-icon {
    z-index: 2;
    width: auto;
    height: 32rem;
  }

  .overlay {
    z-index: 3;
    position: absolute;
    bottom: 8rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 1.5rem;
  }
}

.gradient-border {
  position: relative;
  border-radius: 1rem;

  &:before {
    content: '';
    position: absolute;
    inset: 0;
    padding: 1px;
    z-index: -1;
    border-radius: 1rem;
    background: var(--landing-border-gradient);

    -webkit-mask:
      linear-gradient(#fff 0 0) content-box,
      linear-gradient(#fff 0 0);
    mask:
      linear-gradient(#fff 0 0) content-box,
      linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor;
    mask-composite: exclude;
  }
}

.section-header {
  text-align: center;
  margin: 2rem;

  .section-label {
    margin: 1.5rem auto;
    width: fit-content;
    padding: 1rem 1.5rem;
    border-radius: 0.5rem;
    font-weight: 700;
    font-size: 1rem;
    line-height: 125%;

    &.green {
      background: var(--landing-green-label-bg);
      color: var(--landing-green-label);
    }

    &.blue {
      background: var(--landing-blue-label-bg);
      color: var(--landing-blue-label);
    }
  }

  .section-tagline,
  .section-description {
    font-weight: 400;
    font-size: 1.25rem;
    line-break: loose;
    line-height: 125%;
    max-width: 50rem;
  }

  .section-tagline {
    margin: 0 auto;
    color: var(--landing-color-heading);
  }

  .section-description {
    margin: 0.375rem auto;
    color: var(--landing-color-subheading);
  }
}

.main-header {
  color: var(--landing-color-heading);
  font-size: 2.25rem;
  font-weight: 600;
  line-height: 100%;
  margin: 40 0 0.25rem;
  width: 100%;
}

.main-header-strong {
  display: block;
  margin-top: 1.25rem;
  font-weight: 600;
  background-color: #00bd3c;
  background-image: linear-gradient(180deg, #a7d0ff 0%, var(--color-brand) 60%);
  background-size: 100%;
  background-clip: text;
  -webkit-text-fill-color: transparent;
  -moz-text-fill-color: transparent;
  color: transparent;
  white-space: nowrap;
}

.animate-strong {
  height: 1.2em;
  line-height: 120%;
  position: relative;
  overflow: hidden;
  display: flex;
  justify-content: center;

  >span {
    position: absolute;
    top: 0;
    animation: slide 14s infinite;

    @media (prefers-reduced-motion) {
      animation-play-state: paused !important;
    }
  }

  @keyframes slide {

    0%,
    12.5% {
      top: 0;
    }

    14.3%,
    26.8% {
      top: -1.2em;
    }

    28.6%,
    41.1% {
      top: -2.4em;
    }

    42.9%,
    55.4% {
      top: -3.6em;
    }

    57.2%,
    69.7% {
      top: -4.8em;
    }

    71.5%,
    84% {
      top: -6em;
    }

    85.8%,
    98.3% {
      top: -7.2em;
    }

    100% {
      top: -8.4em;
    }
  }
}

@media screen and (min-width: 560px) {
  .landing-hero {
    h2 {
      font-size: 1.5rem;
    }
  }

  .users-section-outer {
    .users-section {
      .feature-blob {
        .blob-text {
          h3 {
            font-size: 3rem;
          }

          p {
            font-size: 1.5rem;
          }
        }

        .blob-demonstration {
          .demo-search {
            min-height: 37.5rem;
          }

          .notifs-demo {
            .notifications .notification .avatar {
              width: 5rem;
              height: 5rem;
            }
          }
        }
      }
    }
  }

  .logo-banner {
    padding: 3rem 1rem 3.75rem 1rem;

    .overlay {
      bottom: 3.5rem;
    }
  }

  .section-header {

    .section-tagline,
    .section-description {
      font-size: 1.5rem;
    }
  }

  .main-header {
    font-size: 4rem;
  }
}

@media screen and (min-width: 1024px) {
  .landing-hero {
    h2 {
      font-size: 1.625rem;
    }

    margin-top: -5rem;
    padding: 11.25rem 1rem 12rem;
  }

  .users-section-outer {
    .users-section {
      .feature-blob {
        .blob-text {
          h3 {
            font-size: 4rem;
          }

          p {
            font-size: 1.625rem;
          }
        }
      }
    }
  }

  .creator-section {
    .features {
      margin-top: 7rem;
      row-gap: 5.5rem;

      .feature {
        min-height: 12rem;

        .icon {
          margin-bottom: 1.25rem;
          width: 5rem;
          height: 5rem;

          svg {
            width: 2.25rem;
          }
        }

        .additional-label {
          position: absolute;
          top: 12px;
          left: 112px;
          margin-bottom: 0;
        }

        h3 {
          font-size: 1.75rem;
        }

        p {
          font-size: 1.25rem;
        }
      }
    }
  }

  .logo-banner {
    padding: 4rem 1rem 6.75rem 1rem;

    .overlay {
      bottom: 5rem;
    }
  }

  .section-header {

    .section-tagline,
    .section-description {
      font-size: 1.625rem;
    }
  }

  .main-header {
    font-size: 5.25rem;
  }
}
</style>
