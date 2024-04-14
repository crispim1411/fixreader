import "./Details.css";

type DetailsProps = {
    line: FixMsg
}

const Details = ({ line } : DetailsProps) => {
    line.values = line.values.sort((a, b) => Number(a.tag) > Number(b.tag) ? 1 : -1)
    const details = (msg: FixMsg ) => {
        return (
            <table className="details-table">
                <tbody>
                {
                    msg.values.map((field, index) => 
                        (<tr key={`details_${line.id}_${index}`}>
                            <td>{field.tag}</td>
                            <td>{field.title}</td>
                            <td>{field.value}</td>
                            <td>{field.required ? "Y" : ""}</td>
                        </tr>)
                    )
                }
                </tbody>
            </table>
        )
    }

    return details(line!) 
}

export default Details;