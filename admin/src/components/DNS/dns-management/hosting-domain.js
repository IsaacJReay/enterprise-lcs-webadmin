import React, { useState, useEffect } from "react";
import { Checkbox, Form, Collapse, Button, message } from "antd";
import { CaretRightOutlined } from "@ant-design/icons";
import axios from "axios";

const { Panel } = Collapse;

const HostingDomain = () => {
  // ------state----------
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();

  // ------token ------

  const getToken = localStorage.getItem("token");
  const auth = {
    Authorization: "Bearer " + getToken,
  };

  // ----------get data -------------

  useEffect(() => {
    axios
      .get(
        `http://10.42.0.188:8080/private/api/settings/dns/zone_records/status/${key}`,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )
      .then((res) => {
        setLoading(true);
        const { status } = res.data;
        form.setFieldsValue({
          status: status,
        });
        setTimeout(() => {
          setLoading(false);
        }, 1000);
      })
      .catch((err) => console.log(err));
  }, []);

  // ---------- hosting domain ----------
  const handleApply = (data) => {
    const inputData = {
      id: key,
      status: data.status,
    };
    axios
      .put(
        "http://10.42.0.188:8080/private/api/settings/dns/status/update",
        inputData,
        {
          headers: {
            "content-type": "application/json",
            ...auth,
          },
        }
      )

      .then((res) => {
        if (res.data.operation_status === "Success") {
          setLoading(true);
          message.success("Successful!");
          setLoading(false);
          window.location.reload();
        } else {
          setLoading(true);
          message.error("Operation Failed! ");
          setLoading(false);
        }
      })

      .catch((err) => {
        setTimeout(() => {
          setLoading(false);
        }, 1000);
        console.log(err);
        message.error("Operation Failed! ");
      });
  };

  return (
    <React.Fragment>
      <div className="container">
        <div className="container-header">
          <h1>Domain Setting</h1>
        </div>
        <hr />

        <Collapse
          bordered={false}
          expandIcon={({ isActive }) => (
            <CaretRightOutlined rotate={isActive ? 90 : 0} />
          )}
        >
          <Panel header="Hosting Domain!" key="1">
            <Form onFinish={handleApply} form={form}>
              <Form.Item name="status" valuePropName="checked">
                <Checkbox style={{ color: "red ", paddingLeft: "20px" }}>
                  Domain is hosting!
                </Checkbox>
              </Form.Item>
              <Form.Item>
                <Button
                  type="primary"
                  className="button-apply2"
                  htmlType="submit"
                >
                  Apply
                </Button>
              </Form.Item>
            </Form>
          </Panel>
        </Collapse>
      </div>
    </React.Fragment>
  );
};

export default HostingDomain;
