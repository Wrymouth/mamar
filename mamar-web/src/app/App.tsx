import { Provider as SpectrumProvider, defaultTheme, Grid, View } from "@adobe/react-spectrum"

import Header from "./header/Header"
import Main from "./Main"
import { RootProvider } from "./store/dispatch"
import { MupenProvider } from "./util/hooks/useMupen"
import useRomData, { RomDataProvider } from "./util/hooks/useRomData"

export function RomDataConsumer() {
    const romData = useRomData()

    return <MupenProvider romData={romData}>
        <Grid
            areas={[
                "header",
                "content",
            ]}
            columns={["1fr"]}
            rows={["auto", "1fr"]}
            height="100vh"
        >
            <View gridArea="header">
                <Header />
            </View>
            <View gridArea="content" overflow="auto">
                <Main />
            </View>
        </Grid>
    </MupenProvider>
}

export default function App() {
    return <RootProvider>
        <SpectrumProvider theme={defaultTheme}>
            <View UNSAFE_className="App">
                <RomDataProvider>
                    <RomDataConsumer />
                </RomDataProvider>
            </View>
        </SpectrumProvider>
    </RootProvider>
}