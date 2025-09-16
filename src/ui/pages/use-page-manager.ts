import {ref} from "vue";
import {ConsulClient} from "../../core/tauri-bride.ts";

export type ServerListPageInfo = {
    name: 'server-list',
    props: {},
}

export type EditorPageInfo = {
    name: 'editor',
    props: {
        consulClient: ConsulClient,
    },
}

export type SmartUIPageInfo = {
    name: 'smart-ui',
    props: {
        consulClient: ConsulClient,
    },
}

export type PageInfo = ServerListPageInfo | EditorPageInfo | SmartUIPageInfo;
export type ChangePageFn = (newPageInfo: PageInfo) => void;

export function usePageManager() {
    const pageInfo = ref<PageInfo>({
        name: 'server-list',
        props: {},
    });

    const changePage = (newPageInfo: PageInfo) => {
        pageInfo.value = newPageInfo;
    };

    return {
        pageInfo,
        changePage,
    }
}