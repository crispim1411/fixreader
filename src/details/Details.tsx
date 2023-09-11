import { emit, listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";
import "./Details.css";

const Details = () => {
    const [loading, setLoading] = useState(true);
    const [line, setLine] = useState<FixMsg | null>(null);

    useEffect(() => {
        console.log("starting...");
        if (loading) {
            emit('detailsInfoRequest', {});
            listen('detailsInfoResponse', (event: any) => {
                setLine(event.payload.line);
                setLoading(false);
            });
        }
    }, []); 

    const details = (msg: FixMsg ) => {
        return (
            <table className="details-table">
                <tbody>
                {
                    msg.fields.map((field, index) => 
                        (<tr key={index}>
                        <td> {field.tag}</td>
                        <td>{field.value}</td>
                        </tr>)
                    )
                }
                </tbody>
            </table>
        )
    }

    return loading 
        ? "loading..." 
        : details(line!) 
}

export default Details;