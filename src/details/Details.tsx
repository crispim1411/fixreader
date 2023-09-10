import { emit, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import "./Details.css";

const Details = () => {
    const [loading, setLoading] = useState(true);
    const [line, setLine] = useState<FixMsg | null>(null);

    useEffect(() => {
        if (loading) {
            emit('detailsInfoRequest', {});
            listen('detailsInfoResponse', (event: any) => {
                console.log(event.payload.line);
                setLine(event.payload.line);
                setLoading(false);
            });
        }
    }, []); 

    return (
        <>
        {
            loading
                ? "loading..."
                : line?.fields
                    .map((el) => (<li key={el.tag}> {el.tag}: {el.value} </li>))
        }
        </>
    )
}

export default Details;