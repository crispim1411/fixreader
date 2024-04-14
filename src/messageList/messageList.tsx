import { useState } from "react";
import Details from "../details/Details";
import "./messageList.css"

interface MessageListProps {
    lines: FixMsg[],
    handleRemoveLine: (index: number) => void
  }
  
const MessageList = ({lines, handleRemoveLine} : MessageListProps)  => {
    const [openedItem, setOpenedItem] = useState(-1);
    
    return (
      <>
        <table className="converted-table">
          <thead>
            <tr>
              <th>Converted Lines</th>
            </tr>
          </thead>
          <tbody>
          {
            lines.map(msg => (
              <><tr key={msg.id}>
                <td className="fixLine" 
                  style={{"cursor": "pointer"}}
                  onClick={(_) => openedItem == msg.id ? setOpenedItem(-1) : setOpenedItem(msg.id)}>
                  {
                    msg.values
                      .map(field => field.tag + ": " + field.value).join(" | ")
                  }
                </td>
                <td>
                  <button className="removeLine" onClick={() => handleRemoveLine(msg.id)}>X</button>
                </td>
              </tr>
              <div className="collapsible " style={{"height": openedItem == msg.id ? "auto" : 0, "overflow": "clip", "cursor": "text"}}>
                  <Details line={msg}/>
              </div></>
            ))
          }
          </tbody>
        </table>
    </>
    )
}
export default MessageList;