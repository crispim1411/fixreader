import "./Details.css";

type DetailsProps = {
    line: FixMsg
}

const Details = ({ line } : DetailsProps) => {
    const details = (msg: FixMsg ) => {
        return (
            <table className="details-table">
                <tbody>
                {
                    msg.values.map((field, index) => 
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

    return details(line!) 
}

export default Details;